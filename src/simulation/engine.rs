use crate::simulation::object;

pub struct ObjectSnapshot {
    pub pos: na::Vector3<f32>, 
    pub velocity: na::Vector3<f32>, 
    pub acceleration: na::Vector3<f32>, 
}

pub struct EngineState {
    // FIXME: Should be a Vector of mappings of objectId->ObjectSnapshot
    pub engine_snapshots: Vec<Vec<ObjectSnapshot>>,
}

pub struct Engine {
    pub state: EngineState,
    pub objects: Vec<Box<dyn object::MovingPoint>>,
}

impl EngineState {
    pub fn new() -> EngineState {
        EngineState {
            engine_snapshots: Vec::new(),
        }
    }
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            state: EngineState::new(),
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: Box<dyn object::MovingPoint> ) {
        self.objects.push(object);
    }

    fn run_once(&mut self, tick: u32) {
        for obj in &mut self.objects {
            // FIXME: configure time_elapsed per tick
            obj.update(1);
        }
        // FIXME: Currently everytime it ticks, it'll print
        // obj pos, velocity, acceleration
        let mut obj_snapshots = Vec::new();
        for obj in self.objects.iter() {
            let ss = ObjectSnapshot {
                pos: obj.get_pos(),
                velocity: obj.get_velocity(),
                acceleration: obj.get_acceleration(),
            };
            // FIXME: Should have object id so we can tell which pos, velocity, accel is
            // being printed
            println!("tick={}, pos={},velocity={},accel={}",
                     tick, ss.pos, ss.velocity, ss.acceleration);
            obj_snapshots.push(ss);
        }
        self.state.engine_snapshots.push(obj_snapshots);
    }

    pub fn run_for(&mut self, n_loops: u32) {
        let mut i = 0;
        while i < n_loops {
            self.run_once(i);
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::simulation::engine;

    // run_for is populating the engine_snapshots vec
    // with snapshots of each object whenever it ticks
    // (which is just  the arg passed into run_for)
    #[test]
    fn test_engine_run_for() {
        let mut engine = engine::Engine::new();
        assert_eq!(0, engine.state.engine_snapshots.len());
        engine.run_for(1);
        assert_eq!(1, engine.state.engine_snapshots.len());
        engine.run_for(4);
        assert_eq!(5, engine.state.engine_snapshots.len());
    }
}
