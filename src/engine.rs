pub struct EngineSnapshot {
    pub pos: na::Vector3<f32>, 
}
pub struct EngineState {
    pub engine_time: u32,
    pub snapshots: Vec<EngineSnapshot>
}
pub struct Engine {
    pub state: EngineState,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            state: EngineState { 
                engine_time: 0,
                snapshots: vec![EngineSnapshot {
                    pos: na::Vector3::new(0., 0., 0.),
                }],
            }
        }
    }

    fn run_once(&mut self) {
        self.state.engine_time += 1;
    }

    pub fn run_for(&mut self, n_loops: u32) {
        let mut i = 0;
        while i < n_loops {
            self.run_once();
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::engine;

    #[test]
    fn test_engine_new() {
        let engine = engine::Engine::new();
        assert_eq!(1, engine.state.snapshots.len());
    }

    #[test]
    fn test_engine_run_for() {
        let mut engine = engine::Engine::new();
        assert_eq!(0, engine.state.engine_time);
        engine.run_for(1);
        assert_eq!(1, engine.state.engine_time);
        engine.run_for(4);
        assert_eq!(5, engine.state.engine_time);
        
    }
}
