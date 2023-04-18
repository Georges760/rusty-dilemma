use embassy_executor::Spawner;
use embassy_time::Duration;

pub mod distributors;
pub mod transmissions;

pub fn init(spawner: &Spawner) {
    transmissions::init_pool();

    spawner.must_spawn(distributors::from_usb_distributor());
}

pub struct TransmittedMessage<T> {
    pub msg: T,
    pub timeout: Option<Duration>,
}

pub fn low_latency_msg<T>(msg: T) -> TransmittedMessage<T> {
    TransmittedMessage { msg, timeout: Some(Duration::from_millis(2)) }
}

pub fn reliable_msg<T>(msg: T) -> TransmittedMessage<T> {
    TransmittedMessage { msg, timeout: Some(Duration::from_millis(20)) }
}

pub fn unreliable_msg<T>(msg: T) -> TransmittedMessage<T> {
    TransmittedMessage { msg, timeout: None }
}
