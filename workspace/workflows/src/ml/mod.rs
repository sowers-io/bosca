use candle_core::Device;

pub mod whisper;
mod multilingual;

use candle_core::utils::{cuda_is_available, metal_is_available};
use crate::activity;

pub fn device(cpu: bool) -> Result<Device, activity::Error> {
    if cpu {
        Ok(Device::Cpu)
    } else if cuda_is_available() {
        match Device::new_cuda(0) {
            Ok(device) => Ok(device),
            Err(err) => {
                println!("falling back to CPU: {}", err);
                Ok(Device::Cpu)
            }
        }
    } else if metal_is_available() {
        match Device::new_metal(0) {
            Ok(device) => Ok(device),
            Err(err) => {
                println!("falling back to CPU: {}", err);
                Ok(Device::Cpu)
            }
        }
    } else {
        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        {
            println!(
                "Running on CPU, to run on GPU(metal), build this example with `--features metal`"
            );
        }
        #[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
        {
            println!("Running on CPU, to run on GPU, build this example with `--features cuda`");
        }
        Ok(Device::Cpu)
    }
}