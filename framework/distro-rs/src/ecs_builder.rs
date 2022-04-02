

pub struct EcsBuilder {
    fns: Vec<fn()>
}

impl EcsBuilder {
    pub fn with_system(mut self, f: fn()) -> EcsBuilder {
        self.fns.push(f);
        self
    }

    pub fn execute(self) -> Result<(), String> {
        for f in self.fns {
            f();
        }
        Result::Ok(())
    }
}

pub fn use_ecs() -> EcsBuilder {
    EcsBuilder{ fns: Vec::new() }
}