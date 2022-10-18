//use ec_gpu_gen::rust_gpu_tools::Device;

use rust_gpu_tools::Device;

fn main() {
    let devices = Device::all();
    println!("device: {:?}", devices)
}

