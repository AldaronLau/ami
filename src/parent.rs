// "ami" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

use std::{ collections::BinaryHeap, cmp::Ordering, ptr::NonNull };

/// Id is a reverse order u32 (0 is bigger than 1).
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Id(pub u32);

impl PartialOrd for Id {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		other.0.partial_cmp(&self.0)
	}
}

// Id is ordered backwards for minheap capabilities.
impl Ord for Id {
	fn cmp(&self, other: &Id) -> Ordering {
		match self.0.partial_cmp(&other.0).unwrap() {
			Ordering::Greater => Ordering::Less,
			Ordering::Less => Ordering::Greater,
			Ordering::Equal => Ordering::Equal,
		}
	}
}

struct HeapParent<T, U> {
	// The children
	v: Vec<*mut HeapChild<T,U>>,
	// MinHeap of unused ids.
	o: BinaryHeap<Id>,
	// TypeA's data fields.
	d: T,
}

/// A structure that has ownership on `Child`s
pub struct Parent<T, U: PseudoDrop<T = T>> {
	// The Parent Data on the heap.
	heap: *mut HeapParent<T, U>
}

// TypeB depends on TypeA and can only exist while TypeA exists.  It can also
// access TypeA's fields.
struct HeapChild<T, U>(
	// TypeB's parent pointer, null if Child is pseudo-dropped by TypeA.
	Option<(NonNull<HeapParent<T, U>>, U)>,
);

/// A structure that is owned by a `Parent`.
pub struct Child<T, U: PseudoDrop<T = T>> {
	// The Child Data on the heap.
	heap: *mut HeapChild<T, U>,
	// The index within the parent.
	id: Id,
}

impl<T, U> Parent<T, U> where U: PseudoDrop<T = T> {
	/// Create a new `Parent` 
	pub fn new(data: T) -> Self {
		let heap_parent: Box<HeapParent<T, U>> = Box::new(HeapParent {
			v: vec![],
			o: BinaryHeap::new(),
			d: data,
		});

		Parent {
			heap: unsafe { ::std::mem::transmute(heap_parent) },
		}
	}

	/// Get the parent data.
	pub fn data<'a>(&'a self) -> &'a mut T {
		unsafe { &mut (*self.heap).d }
	}
}

impl<T, U> Drop for Parent<T, U> where U: PseudoDrop<T = T> {
	fn drop(&mut self) {
		for i in unsafe { (*self.heap).v.iter() } {
			if unsafe { (**i).0.is_some() } {
				unsafe {
					(**i).0.as_mut().unwrap().1
						.pdrop(&mut self.data());
				}
				unsafe { (**i).0 = None };
			}
		}

		let _ = unsafe {
			::std::mem::transmute::<_, Box<HeapParent<T, U>>>(self.heap)
		};
	}
}

impl<T, U> Child<T, U> where U: PseudoDrop<T = T> {
	/// Create a new `Child` for the `Parent`.
	pub fn new(parent: &Parent<T, U>, data: U) -> Self {
		let heap_child = Box::new(HeapChild(Some(
			(NonNull::new(parent.heap).unwrap(), data)
		)));

		let heap_pointer = unsafe { ::std::mem::transmute(heap_child) };

		let id = unsafe {
			// Look for an open spot.
			if let Some(id) = (*parent.heap).o.pop() {
				(*parent.heap).v[id.0 as usize] = heap_pointer;
				id
			} else {
				// Add at the end.
				let a = (*parent.heap).v.len() as u32;
				(*parent.heap).v.push(heap_pointer);
				Id(a)
			}
		};

		Child {
			heap: heap_pointer,
			id,
		}
	}

	fn heap_parent<'a>(&'a self) -> &'a HeapParent<T, U> {
		unsafe { (*self.heap).0.as_mut().unwrap().0.as_mut() }
	}

	fn heap_parent_mut<'a>(&'a mut self) -> &'a mut HeapParent<T, U> {
		unsafe { (*self.heap).0.as_mut().unwrap().0.as_mut() }
	}

	/// Get the `Parent`'s data.
	pub fn parent<'a>(&'a self) -> &'a T {
		&self.heap_parent().d
	}

	/// Get the `Parent`'s data (mutable).
	pub fn parent_mut<'a>(&'a mut self) -> &'a mut T {
		&mut self.heap_parent_mut().d
	}

	/// Get the `Child`'s data.
	pub fn data<'a>(&'a self) -> &'a U {
		unsafe { &(*self.heap).0.as_mut().unwrap().1 }
	}

	/// Get the `Child`'s data (mutable).
	pub fn data_mut<'a>(&'a mut self) -> &'a mut U {
		unsafe { &mut (*self.heap).0.as_mut().unwrap().1 }
	}

	/// Get the `Id` of this `Child`
	pub fn id(&self) -> Id {
		self.id
	}
}

impl<T, U> Drop for Child<T, U> where U: PseudoDrop<T = T> {
	fn drop(&mut self) {
		// If it hasn't already been pseudo-dropped.
		if unsafe { &(*self.heap).0 }.is_some() {
			let id = self.id;
			self.heap_parent_mut().o.push(id);
			unsafe {
				(*self.heap).0.as_mut().unwrap().1
					.pdrop(&mut self.heap_parent_mut().d);
			}
			unsafe { (*self.heap).0 = None };
		}
	}
}

/// A trait for the drop method of a child type, where it can access it's parent
pub trait PseudoDrop {
	/// The type of the parent data.
	type T;

	/// The drop method with parent data as a parameter.
	fn pdrop(&mut self, parent: &mut Self::T);
}
