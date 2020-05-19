use test::Bencher;
use crate::utils::queue::sorted::{SortedQueue, VecBackedSortedQueue, Weighted};
use crate::search::Direction;
use crate::utils::queue::QueueStrategy;
use rand::random;
use std::sync::mpsc;
use crate::utils::queue::fifo::FIFOQueue;

#[repr(transparent)]
pub struct StaticallyWeighed(usize);

impl Weighted for StaticallyWeighed {
	fn weigh(&self) -> usize {
		self.0
	}

	fn heuristic_weight(&mut self) -> &mut usize {
		unimplemented!("unused function")
	}

	fn direction(&self) -> Option<Direction> {
		None
	}
}

#[bench]
fn bheap_sorted_queue(b: &mut Bencher) {
	let mut queue = SortedQueue::init();

	for i in 0..=1000 {
		queue.queue(StaticallyWeighed(i));
	}

	b.iter(|| {
		let i = random::<u8>();
		queue.queue(StaticallyWeighed(i as usize));
		let _ = queue.dequeue();
	})
}

#[bench]
fn vec_backed_sorted_queue(b: &mut Bencher) {
	let mut queue = VecBackedSortedQueue::init();

	for i in 0..=1000 {
		queue.queue(StaticallyWeighed(i));
	}

	b.iter(|| {
		let i = random::<u8>();
		queue.queue(StaticallyWeighed(i as usize));
		let _ = queue.dequeue();
	})
}

#[bench]
fn fifo_queue(b: &mut Bencher) {
	let mut queue = FIFOQueue::init();

	for i in 0..=1000 {
		queue.queue(StaticallyWeighed(i));
	}

	b.iter(|| {
		let i = random::<u8>();
		queue.queue(StaticallyWeighed(i as usize));
		let _ = queue.dequeue();
	})
}

#[bench]
fn mpsc_queue(b: &mut Bencher) {
	let (tx, rx) = mpsc::channel();

	for i in 0..=1000 {
		tx.send(StaticallyWeighed(i)).expect("broken queue");
	}

	b.iter(|| {
		let i = random::<u8>();
		tx.send(StaticallyWeighed(i as usize)).expect("broken queue");
		let _ = rx.recv();
	})
}
