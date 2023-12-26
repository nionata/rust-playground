use std::sync::Mutex;

#[derive(Debug)]
struct Simple<'slice> {
    i: Mutex<&'slice [u8]>,
}

impl<'slice> Simple<'slice> {
    fn incr(&self) {
        let mut guard = self.i.lock().unwrap();
        *guard = &guard[1..];
    }
}

fn main() {
    let mut s = Simple {
        i: Mutex::new(&[1, 2, 3])
    };
    
    println!("{s:?}");
    
    s.incr();
    
    println!("{s:?}");
    
    s.incr();
    
    println!("{s:?}");
    
    s.incr();
    
    println!("{s:?}");
}
