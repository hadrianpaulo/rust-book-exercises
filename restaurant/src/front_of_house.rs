
pub mod hosting {
    enum QueueStatus {
        Waiting,
        Seated,
    }

    pub fn add_to_waitlist() {
        QueueStatus::Waiting;
    }

    fn seat_at_table() {
        QueueStatus::Seated;
    }
}

pub mod serving {
    fn take_order() {}

    pub fn serve_order() {}

    fn take_payment() {}
}
