use rand::Rng;

struct MyController {
    vehicle: MyVehicle
}

impl MyController {
    fn command(&mut self) -> Result<()> {
        if self.vehicle.healthy() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Bad"))
        }
    }
}

struct MyVehicle {
    listener: MyListener,
}

impl MyVehicle {
    fn healthy(&mut self) -> bool {
        self.listener.watch() == State::Healthy
    }
}

#[derive(PartialEq)]
enum State {
    Healthy,
    NotHealthy,
}

struct MyListener {
    rng: rand::rngs::ThreadRng,
}

impl MyListener {
    fn watch(&mut self) -> State {
        let value = self.rng.gen_range(0..2);
        
        if value == 0 {
            State::Healthy
        } else {
            State::NotHealthy
        }
    }
}

fn main() {
    let mut c = MyController {
        vehicle: MyVehicle {
            listener: MyListener {
                rng: rand::thread_rng()
            }
        }
    };
    
    println!("{:?}", c.command())
}
