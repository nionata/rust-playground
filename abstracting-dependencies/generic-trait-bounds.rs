use anyhow::Result;
use rand::Rng;

trait Controller {
    fn command(&mut self) -> Result<()>;
}

trait Vehicle {
    fn healthy(&mut self) -> bool;
}

trait Listener {
    fn watch(&mut self) -> State;
}

struct MyController<V> {
    vehicle: V,
}

impl<V: Vehicle> Controller for MyController<V> {
    fn command(&mut self) -> Result<()> {
        if self.vehicle.healthy() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Bad"))
        }
    }
}

struct MyVehicle<L> {
    listener: L,
}

impl<L: Listener> Vehicle for MyVehicle<L> {
    fn healthy(&mut self) -> bool {
        self.listener.watch() == State::Healthy
    }
}

struct UnhealthyVehicle {}
 
impl Vehicle for UnhealthyVehicle {
    fn healthy(&mut self) -> bool {
        false
    }
}

#[derive(PartialEq, Debug)]
enum State {
    Healthy,
    NotHealthy,
}

struct RandomListener {
    rng: rand::rngs::ThreadRng,
}

impl Listener for RandomListener {
    fn watch(&mut self) -> State {
        let value = self.rng.gen_range(0..2);

        if value == 0 {
            State::Healthy
        } else {
            State::NotHealthy
        }
    }
}

struct NotHealthyListener {}

impl Listener for NotHealthyListener {
    fn watch(&mut self) -> State {
        State::NotHealthy
    }
}

fn main() {
    let mut l = RandomListener {
        rng: rand::thread_rng(),
    };
    println!("RandomListener - {:?}", l.watch());

    let mut v = MyVehicle {
        listener: RandomListener {
            rng: rand::thread_rng(),
        },
    };
    println!("MyVehicle RandomListener - {:?}", v.healthy());

    let mut v2 = MyVehicle {
        listener: NotHealthyListener {},
    };
    println!("MyVehicle NotHealthyListener - {:?}", v2.healthy());

    let mut c = MyController {
        vehicle: MyVehicle {
            listener: RandomListener {
                rng: rand::thread_rng(),
            },
        },
    };
    println!("MyController MyVehicle RandomListener - {:?}", c.command());

    let mut c2 = MyController {
        vehicle: MyVehicle {
            listener: NotHealthyListener {},
        },
    };
    println!(
        "MyController MyVehicle NotHealthyListener - {:?}",
        c2.command()
    );

    let mut c3 = MyController {
        vehicle: UnhealthyVehicle {},
    };
    println!("MyController UnhealthyVehicle - {:?}", c3.command());
}
