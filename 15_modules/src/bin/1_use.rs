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



use std::collections::HashMap;



fn main() {
    instrument::clarinet();
    instrument::clarinet();
    instrument::clarinet();



    performance_group::clarinet_trio();



    let mut map = HashMap::new();

    map.insert(1, 2);



//    use std::fmt;
//    use std::io;

//    fn function1() -> fmt::Result {
//        Result::Ok(())
//    }
//    fn function2() -> io::Result<()> {
//        Result::Ok(())
//    }



    use std::fmt::Result;
    use std::io::Result as IoResult;

    fn function1() -> Result {
        std::result::Result::Ok(())
    }
    fn function2() -> IoResult<()> {
        std::result::Result::Ok(())
    }

}

