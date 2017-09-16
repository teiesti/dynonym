// use errors::*;   // TODO necessary?

use std::path::Path;

pub struct State {
    // TODO add fields
}

impl State {
    pub fn build() -> StateBuilder {
        StateBuilder::default()
    }
}

pub struct StateBuilder {
    // TODO add fields
}

impl StateBuilder {
    pub fn config<P: AsRef<Path>>(self, _config: P) -> Self {
        unimplemented!()
    }

    pub fn config_write_through(self, _val: bool) -> Self {
        unimplemented!()
    }

    pub fn pid<P: AsRef<Path>>(self, _pid: P) -> Self {
        unimplemented!()
    }

    // TODO Add DNS UPDATE provider

    pub fn finalize(self) -> State {
        unimplemented!()
    }
}

impl Default for StateBuilder {
    fn default() -> Self {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO Remove!
    #[test]
    #[ignore]
    fn it_works() {
        let _state = State::build()
            .config("/etc/dynonym.toml")
            .config_write_through(true)
            .pid("/var/run/dynonym.pid")
            .finalize()
        ;
    }
}
