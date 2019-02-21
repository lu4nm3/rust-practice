mod sound {
    pub mod instrument {
        pub fn clarinet() {

        }
    }
}

// Using absolute path
//use crate::sound::instrument;

// Using relative path
use self::sound::instrument;



mod performance_group {
    use crate::sound:: instrument;

    pub fn clarinet_trio() {
        instrument::clarinet();
        instrument::clarinet();
        instrument::clarinet();
    }
}

fn main() {
    instrument::clarinet();
    instrument::clarinet();
    instrument::clarinet();



    performance_group::clarinet_trio();
}

