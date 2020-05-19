pub mod sorted;
pub mod fifo;
#[cfg(test)]
mod bench;

/// A queue adapter which allows for alternative queueing strategies such as FIFO
pub trait QueueStrategy<T> {
	fn queue(&mut self, item: T);

	fn dequeue(&mut self) -> Option<T>;
}