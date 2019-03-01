mod sound {
    pub mod instrument {
        pub fn clarinet() {

        }
    }
}

mod performance_group {
    pub use crate::sound::instrument;

    pub fn clarinet_trio() {
        instrument::clarinet();
        instrument::clarinet();
        instrument::clarinet();
    }
}

fn main() {
    performance_group::clarinet_trio();

    use sound::instrument;
    instrument::clarinet();

    performance_group::instrument::clarinet();
}