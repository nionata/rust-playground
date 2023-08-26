trait Handler {
    fn run(&self) -> bool;
}

struct TrueHandler {}

impl Handler for TrueHandler {
    fn run(&self) -> bool {
        return true;
    }
}

struct FalseHandler {}

impl Handler for FalseHandler {
    fn run(&self) -> bool {
        return false;
    }
}

struct Dispatcher<T: Handler, U: Handler> {
    true_handler: T,
    false_handler: U,
}

impl<T: Handler, U: Handler> Dispatcher<T, U> {
    fn run(&self) {
        print_one(&self.true_handler);
        print_two(&self.false_handler);
    }
}

fn main() {
    let t = &TrueHandler{};
    let f = &FalseHandler{};
    
    print_one(t);
    print_one(f);
    
    print_two(t);
    print_two(f);
    
    let d = Dispatcher {
        true_handler: TrueHandler{},
        false_handler: FalseHandler{}
    };
    d.run();
}

fn print_one(h: &impl Handler) {
    println!("this is {}", h.run())
}

fn print_two<T: Handler>(h: &T) {
    println!("this is {}", h.run())
}
