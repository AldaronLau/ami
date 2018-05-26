// "ami" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2017  Douglas P. Lau
//  * Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

use std::fmt;
use std::cmp::Ordering;

use Vec3;
use BCube;
use BBox;
use Collider;
use Frustum;

/// An octree is a DAG that can quickly search for points in 3D space.
///
/// The bounding box of the root node contains all points in the octree.
/// If a point outside the bounding box is added, a new root node is created
/// which contains the old root as one of its octants.  This process is repeated
/// until the point is contained.
///
/// The nodes are stored in a vector, and are indexed using a 32-bit node ID.
/// This saves memory over using pointers on 64-bit systems.  Node ID 1 is the
/// first node in the vector.
pub struct Octree<T: Collider> {
	colliders: Vec<T>,
	collider_garbage: Vec<u32>,
	nodes: Vec<Node>,
	garbage: Vec<u32>,
	bcube: BCube,
	root: Id,
	n_colliders: u32,
}

const LINK: usize = 15;			// link to coincident leaf nodes
const LEAF: u32 = 0xFF_FF_FF_FF;	// max u32 value (invalid handle)

pub struct Id(u32);

impl Id {
	/// Get an `Id` that represents nothing.
	fn none() -> Self {
		Id(0)
	}

	/// Does this `Id` represent nothing?
	fn is_none(&self) -> bool {
		self.0 == 0
	}

	/// Does this `Id` represent something?
	fn is_some(&self) -> bool {
		!self.is_none()
	}
}

impl Into<Id> for usize {
	fn into(&self) -> Id {
		Id(self as u32 + 1)
	}
}

impl Into<usize> for Id {
	fn into(&self) -> usize {
		(self.0 - 1) as usize
	}
}

/// A node is either a branch or a leaf.
///
/// A branch can have up to 8 child nodes (each octant adjacent to the center)
/// and 7 objects, plus an optional link to a leaf.
///
/// A leaf can store up to 14 points; the first child must contain a LEAF
/// sentinel value, and the last may link to another leaf node.
///
/// Each node has an implicit bounding box determined by its position in the
/// octree.  The bounding box contains all descendant nodes.
struct Node {
	/// child node handles
	child: [Id; 16],
}

impl fmt::Debug for Node {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.is_leaf() {
			try!(write!(f, "leaf: {:?}", self.leaf_children()));
			let l = self.link();
			if l > 0 {
				try!(write!(f, " link: {:?}", l));
			}
			Ok(())
		} else {
			write!(f, "branch: {:?}", self.child)
		}
	}
}

impl Node {
	/// Create a new leaf node
	fn new_leaf() -> Node {
		Node {
			// no elements, no linking
			child: [
				LEAF, Id::none(), Id::none(), Id::none(),
				Id::none(), Id::none(), Id::none(), Id::none(),
				Id::none(), Id::none(), Id::none(), Id::none(),
				Id::none(), Id::none(), Id::none(), Id::none()
			],
		}
	}

	/// Create a new branch node
	fn new_branch() -> Node {
		Node {
			child: [Id::none(); 16],
		}
	}

	/// Test if a node is a leaf
	fn is_leaf(&self) -> bool {
		self.child[0] == LEAF
	}

	/// Test if a node is a branch
	fn is_branch(&self) -> bool {
		!self.is_leaf()
	}

	/// Find the first open child slot in a branch, None if full.
	fn branch_open_slot(&self) -> Option<usize> {
		assert!(self.is_leaf());
		// Skip 0-7 as that is descending the octree, and skip 15 (link)
		let slot = self.child[8..=14].iter().position(|v| *v == 0);
		if let Some(s) = slot {
			if s < 7 {
				return slot;
			}
		}
		None
	}

	/// Add a collider to a branch node.
	fn branch_add_collider(&mut self, hnd: Id) -> Option<()> {
		assert!(self.is_branch());
		let s = self.branch_open_slot()?;
		self.child[s] = hnd;
	}

	/// Determine which child for a branch point
	fn which_child(c: Vec3, p: Vec3) -> usize {
		match (p.x < c.x, p.y < c.y, p.z < c.z) {
			(true,  true,  true)  => 0,
			(true,  true,  false) => 1,
			(true,  false, true)  => 2,
			(true,  false, false) => 3,
			(false, true,  true)  => 4,
			(false, true,  false) => 5,
			(false, false, true)  => 6,
			(false, false, false) => 7,
		}
	}

	/// Determine which child for a branch bbox, if there is one it fully
	/// fits into.
	fn which_child_bbox(c: Vec3, p: BBox) -> Option<usize> {
		let min = Self::which_child(c, p.min);
		let max = Self::which_child(c, p.max);

		if min == max {
			Some(min)
		} else {
			None
		}
	}

	/// Calculate the center of a child node
	fn child_center(ch: usize, c: Vec3, h: f32) -> Vec3 {
		let h = if h < 0.000001 { 1.0 } else { h };

		match ch {
			0 => Vec3::new(c.x - h, c.y - h, c.z - h),
			1 => Vec3::new(c.x - h, c.y - h, c.z + h),
			2 => Vec3::new(c.x - h, c.y + h, c.z - h),
			3 => Vec3::new(c.x - h, c.y + h, c.z + h),
			4 => Vec3::new(c.x + h, c.y - h, c.z - h),
			5 => Vec3::new(c.x + h, c.y - h, c.z + h),
			6 => Vec3::new(c.x + h, c.y + h, c.z - h),
			7 => Vec3::new(c.x + h, c.y + h, c.z + h),
			a => panic!("ch must be 0-7, not {}", a),
		}
	}

	/// Calculate the bounding box of a child node
	fn child_bcube(ch: usize, bcube: BCube) -> BCube {
		assert!(bcube.half_len > 0.0);
		let half_len = bcube.half_len / 2.0;
		let center = Node::child_center(ch, bcube.center, half_len);
		BCube { center: center, half_len: half_len }
	}
}

impl<T> Octree<T> where T: Collider {
	/// Create a new octree
	pub fn new() -> Octree<T> {
		Octree {
			colliders: vec![],
			collider_garbage: vec![],
			nodes: vec![],
			garbage: vec![],
			bcube: BCube::empty(),
			root: Id::none(),
			n_colliders: 0,
		}
	}

	/// Add a point in the octree
	pub fn add(&mut self, point: T) -> Id {
		// Add to colliders and get the id.
		let id = if let Some(i) = self.collider_garbage.pop() {
			let i = i as usize; // u32 -> usize
			unsafe {
				::std::ptr::copy_nonoverlapping(&point,
					&mut self.points[i], 1);
			}
			::std::mem::forget(point); // don't drop it, it's moved!
			i.into()
		} else {
			self.colliders.push(point);
			Id(self.colliders.len() as u32)
		};

		// Find position in octree for this new collider.
		match self.n_colliders {
			0 => self.add_0(hnd),
			_ => self.add_n(hnd),
		}

		// Increment number of colliders, and return id
		n_colliders += 1;
		id
	}

	/// Add a point when empty
	fn add_0(&mut self, id: Id) {
		// Number of colliders must be 0
		assert!(self.n_colliders == 0);

		// Clear the octree
		self.nodes.clear();
		self.garbage.clear();

		// Make the root bcube contain the bbox of this first point.
		self.bcube = self[id].bbox().into();

		// Build the branch and add a collider.
		let i = self.new_branch();
		self.nodes[i.into()].branch_add_collider(id).unwrap();

		// Set this branch as the root node.
		self.root = i;
	}

	/// Add a point when not empty
	fn add_n(&mut self, id: Id) {
		// Must have colliders already in the octree.
		assert!(self.n_colliders > 0);

		// While the bbox isn't within the root bcube, expand root bcube
		while !self[id].bbox().collide_bcube(self.bcube) {
			self.grow_root(p);
		}

		// Add id inside the root bcube.
		self.add_inside(hnd, self.root, self.bcube);
	}

	/// Grow the root node
	fn grow_root(&mut self, bbox: BBox) {
		// BBox can't collide with bcube when this function is called.
		assert!(!bbox.collide_bcube(self.bcube));
		assert!(self.nodes[self.root.into()].is_branch());

		// Get the old bcube center, to see which octant it goes in.
		let center = self.bcube.center;

		// Extend bcube to attempt to accomodate for bbox.
		// This function is limited to growing twice in size.
		self.bcube.extend(bbox);

		// Create new container branch for old root branch.
		let ch = Node::which_child(self.bcube.center, center);
		let id = self.new_branch();
		self.nodes[id.into()].child[ch] = self.root;
		self.root = id;
	}

	/// Add a point within the bounds
	fn add_inside(&mut self, id: Id, node_id: Id, bcube: BCube) {
		// Calculate bbox for this id.
		let bbox = self[id.into()].bbox();

		// BBox must collide with bcube when this function is called
		assert!(bbox.collide_bcube(bcube));
		assert!(self.nodes[node_id.into()].is_branch());

		// Attempt to add at root first.
		if self.nodes[node_id.into()].branch_add_collider(id)
			.is_none() // Is full
		{
			// Attempt to push relative root colliders down the tree
			for i in 8..=14 {
				if self.add_down(
					self.nodes[node_id.into()].child[i],
					node_id, bcube)
				{
					// If it successfully pushed it the
					// collider down the octree, remove it
					// from it's old location.
					self.nodes[node_id.into()].child[i]
						= Node::zero();
				}
			}

			// Attempt to push this collider (id) down the tree
			if self.add_down(id, node_id, bcube) {
				return;
			}

			// Try again, this time link if failed.
			if self.nodes[node_id.into()].branch_add_collider(id)
				.is_none() // Is full, still!
			{
				let link_id = self.new_leaf();
				self.nodes[node_id.into()].child[LINK]
					= link_id;
			}
		}
	}

	/// Move a collider down the tree, return true if it worked.
	fn add_down(&mut self, id: Id, node_id: Id, bcube: BCube) -> bool {
		// Calculate bbox for this id.
		let bbox = self[id.into()].bbox();

		// can be put on a lower level.
		if let Some(ch) = Node::which_child_bbox(bcube.center, bbox) {
			let j = self.nodes[node_id.into()].child[ch];
			let bc = Node::child_bcube(ch, bcube);

			if j.is_some() {
				// already a branch here, add collider to it.
				self.add_inside(id, j, bc);
			} else {
				// make a branch
				let k = self.new_branch();
				// set branch as the correct child
				self.nodes[node_id.into()].child[ch] = k;
				// Add the collider
				self.nodes[k.into()]
					.branch_add_collider(id)
					.unwrap(); // shouldn't fail.
			}
			true
		} else {
			false
		}
	}

	/// Add a new node
	fn new_node(&mut self, n: Node) -> Id {
		if let Some(i) = self.garbage.pop() {
			let k = i as usize;
			self.nodes[k] = n;
			k.into()
		} else {
			self.nodes.push(n);
			Id(self.nodes.len() as u32)
		}
	}

	/// Add a new leaf node
	fn new_leaf(&mut self) -> Id {
		self.new_node(Node::new_leaf())
	}

	/// Add a new branch node
	fn new_branch(&mut self) -> Id {
		self.new_node(Node::new_branch())
	}
}
