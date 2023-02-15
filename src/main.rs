use metal::{Device, DeviceRef};
use std::rc::Rc;

// includes the dot_product() kernel
const LIB_DATA: &[u8] = include_bytes!("metal/dot_product.metal");

fn main() {
    // the system will assign a GPU to use
    let device: &DeviceRef = &Device::system_default().expect("No device found");
    // represents the library which contains the kernel
    let lib = device.new_library_with_data(LIB_DATA).unwrap();
    // a command queue for sending instructions to the device
    let command_queue = Rc::new(device.new_command_queue());

    let constants = metal::FunctionConstantValues::new();
    let function = lib.get_function("dot_product", todo!());
    // TODO: create two arrays of numbers and pass pointers as constant values.
}

// Reference: https://developer.apple.com/documentation/metal/performing_calculations_on_a_gpu?language=objc
// Reference: https://github.com/andrewmilson/ministark/blob/main/gpu-poly
