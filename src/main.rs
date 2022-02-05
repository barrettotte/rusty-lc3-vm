use lc3::VirtualMachine;
use std::env;
use std::path::Path;
use std::process;

extern crate termios;
use termios::*;

mod lc3;

fn main() {
    // add terminal config for unix
    let stdin = 0;
    let termios = termios::Termios::from_fd(stdin).unwrap();
    let mut new_termios = termios.clone();

    new_termios.c_iflag &= IGNBRK | BRKINT | PARMRK | ISTRIP | INLCR | IGNCR | ICRNL | IXON;
    new_termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode
    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: lc3 <image-path>");
        process::exit(2);
    }
    let img_path = Path::new(&args[1]);
    let mut vm = VirtualMachine::new();

    match vm.load_img(img_path) {
        Ok(_) => vm.run(),
        Err(e) => {
            println!(
                "Failed to load image file '{}'\n{}",
                &img_path.to_str().unwrap_or("?"),
                e
            );
            process::exit(1);
        }
    }
    tcsetattr(stdin, TCSANOW, &termios).unwrap();
}
