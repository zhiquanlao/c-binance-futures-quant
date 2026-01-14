use rtrb::{Consumer, Producer, RingBuffer};

pub type RingProducer<T> = Producer<T>;
pub type RingConsumer<T> = Consumer<T>;

pub struct Ring<T> {
    pub producer: RingProducer<T>,
    pub consumer: RingConsumer<T>,
}

impl<T> Ring<T> {
    pub fn new(capacity: usize) -> Self {
        let (producer, consumer) = RingBuffer::new(capacity);
        Self { producer, consumer }
    }
}
