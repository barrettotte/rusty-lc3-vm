use lc3::VirtualMachine;

mod lc3;

fn main() {
    println!("Hello, world!");
    // load args
    // load file as IO stream

    let mut vm = VirtualMachine::new();
    vm.run();
}
