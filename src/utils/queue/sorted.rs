#[cfg(test)]
use std::sync::mpsc::{Receiver, Sender, self};
use crate::search::Direction;
use crate::utils::queue::QueueStrategy;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

/// Trait to expose the weight of a cursor / node
pub trait Weighted {
	fn weigh(&self) -> usize;

	fn heuristic_weight(&mut self) -> &mut usize;

	fn direction(&self) -> Option<Direction>;
}

/// Glue wrapper for inserting into the binary heap
pub struct CmpWrapper<T> {
	pub inner: T
}

impl<T: Weighted> PartialEq for CmpWrapper<T> {
	fn eq(&self, rhs: &Self) -> bool {
		self.cmp(rhs) == Ordering::Equal
	}
}

impl<T: Weighted> PartialOrd for CmpWrapper<T> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl<T: Weighted> Eq for CmpWrapper<T> {}

impl<T: Weighted> Ord for CmpWrapper<T> {
	fn cmp(&self, rhs: &Self) -> Ordering {
		rhs.inner.weigh().cmp(&self.inner.weigh())
			.then(rhs.inner.direction().cmp(&self.inner.direction()))
	}
}

/// A queue that's sorted from lightest-weighed to heaviest-weighed
pub struct SortedQueue<T> {
	buffer: BinaryHeap<CmpWrapper<T>>,
}

impl<T> QueueStrategy<T> for SortedQueue<T> where
	T: Weighted {
	/// Adds to the queue of objects, O(log n) operation
	fn queue(&mut self, item: T) {
		self.buffer.push(CmpWrapper { inner: item });
	}

	/// Removes the lightest-weighed object, O(log n) operation
	fn dequeue(&mut self) -> Option<T> {
		if let Some(x) = self.buffer.pop() {
			return Some(x.inner);
		}
		None
	}
}

impl SortedQueue<()> {
	pub fn init<T: Weighted>() -> SortedQueue<T> {
		SortedQueue {
			buffer: BinaryHeap::new(),
		}
	}
}

/// Manually sorted vec edition
#[cfg(test)]
pub struct VecBackedSortedQueue<T> {
	buffer: Vec<T>,
	pub tx: Sender<T>,
	rx: Receiver<T>,
}

#[cfg(test)]
impl<T> QueueStrategy<T> for VecBackedSortedQueue<T> where
	T: Weighted {
	fn queue(&mut self, t: T) {
		self.tx.send(t).expect("broken queue")
	}

	fn dequeue(&mut self) -> Option<T> where {
		while let Ok(item) = self.rx.try_recv() {
			self.buffer.push(item);
			self.buffer.sort_by(|a, b| {
				b.weigh().cmp(&a.weigh())
					.then(b.direction().cmp(&a.direction()))
			})
		}

		return self.buffer.pop();
	}
}

#[cfg(test)]
impl VecBackedSortedQueue<()> {
	pub fn init<T>() -> VecBackedSortedQueue<T> {
		let (tx, rx) = mpsc::channel();

		VecBackedSortedQueue {
			buffer: vec![],
			tx,
			rx,
		}
	}
}
