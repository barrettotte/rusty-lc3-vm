use lc3::VirtualMachine;
use std::env;
use std::path::Path;
use std::process;

mod lc3;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: lc3 <image-path>");
        process::exit(2);
    }
    let img_path = Path::new(&args[1]);
    let mut vm = VirtualMachine::new();

    match vm.load_img(img_path) {
        Ok(_) => println!("done."), //vm.run(),
        Err(e) => {
            println!(
                "Failed to load image file '{}'\n{}",
                &img_path.to_str().unwrap_or("?"),
                e
            );
            process::exit(1);
        }
    }
}
