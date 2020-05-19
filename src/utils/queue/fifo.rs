use std::mem;
use crate::utils::queue::QueueStrategy;

/// A queue that's sorted from first to last
pub struct FIFOQueue<T> {
	buffer: Vec<Option<T>>,
	index: usize,
}

impl<T> QueueStrategy<T> for FIFOQueue<T> {
	/// Adds item to queue
	fn queue(&mut self, item: T) {
		self.buffer.push(Some(item));
	}

	/// Removes first item to queue
	fn dequeue(&mut self) -> Option<T> {
		if self.index >= self.buffer.len() {
			return None;
		}

		let shifted = mem::replace(&mut self.buffer[self.index], None);
		self.index += 1;
		return shifted;
	}
}

impl FIFOQueue<()> {
	pub fn init<T>() -> FIFOQueue<T> {
		FIFOQueue {
			// Preallocate a 32KiB buffer for performance
			buffer: Vec::with_capacity(4096),
			index: 0,
		}
	}
}
