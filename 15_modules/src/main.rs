mod sound {
    pub mod instrument {
        pub fn clarinet() {

        }
    }

    mod voice {

    }

    fn guitar() {

    }
}

fn main() {
    // absolute path
    crate::sound::instrument::clarinet();

    // relative path
    sound::instrument::clarinet();
}




mod instrument {
    fn clarinet() {
        super::breath_in();
    }
}

fn breath_in() {

}



mod sound2 {
    mod instrument {
        fn clarinet() {
            super::breath_in();
        }
    }

    fn breath_in() {

    }
}