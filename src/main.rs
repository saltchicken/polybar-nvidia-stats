use nvml_wrapper::enum_wrappers::device::{Clock, TemperatureSensor};
use nvml_wrapper::error::NvmlError;
use nvml_wrapper::{cuda_driver_version_major, cuda_driver_version_minor, Nvml};
use pretty_bytes::converter::convert;

fn main() -> Result<(), NvmlError> {
    let nvml = Nvml::init()?;

    //let cuda_version = nvml.sys_cuda_driver_version()?;

    // Grabbing the first device in the system, whichever one that is.
    // If you want to ensure you get the same physical device across reboots,
    // get devices via UUID or PCI bus IDs.
    let device = nvml.device_by_index(0)?;

    let temperature = device.temperature(TemperatureSensor::Gpu)?;
    let mem_info = device.memory_info()?;
    let mut used_mem = convert(mem_info.used as _);
    // Remove " GB" from used_mem
    used_mem.truncate(used_mem.len() - 3);
    let total_mem = convert(mem_info.total as _);
    let graphics_clock = device.clock_info(Clock::Graphics)?;

    println!(
        "{graphics_clock} {used_mem}/{total_mem} {temperature}",
        temperature = temperature,
        graphics_clock = graphics_clock,
        used_mem = used_mem,
        total_mem = total_mem,
    );
    Ok(())
}
