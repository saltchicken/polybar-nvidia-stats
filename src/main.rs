use nvml_wrapper::enum_wrappers::device::{Clock, TemperatureSensor};
use nvml_wrapper::error::NvmlError;
//use nvml_wrapper::{cuda_driver_version_major, cuda_driver_version_minor, Nvml};
use nvml_wrapper::{Nvml};
//use pretty_bytes::converter::convert;

fn main() -> Result<(), NvmlError> {
    let nvml = Nvml::init()?;

    //let cuda_version = nvml.sys_cuda_driver_version()?;

    // Grabbing the first device in the system, whichever one that is.
    // If you want to ensure you get the same physical device across reboots,
    // get devices via UUID or PCI bus IDs.
    let device = nvml.device_by_index(0)?;

    let temperature = device.temperature(TemperatureSensor::Gpu)?;
    let mem_info = device.memory_info()?;
    //let mut used_mem = convert(mem_info.used as _, unit = Unit::GB);
    // Remove " GB" from used_mem
    //used_mem.truncate(used_mem.len() - 3);
    //let total_mem = convert(mem_info.total as _, unit = Unit::GB);
    //let mut used_mem = mem_info.used / 1024 / 1024 / 1024;
    //let mut total_mem = mem_info.total / 1024 / 1024 / 1024;
    let percentage_mem_used = mem_info.used / mem_info.total;
    let percentage_clock_used = device.clock_info(Clock::Graphics)? / device.max_clock_info(Clock::Graphics)?;
    //let graphics_clock = device.clock_info(Clock::Graphics)?;

    println!(
        //"{graphics_clock} {used_mem}/{total_mem} {temperature}",
        "{percentage_clock_used} {percentage_mem_used} {temperature}",
        temperature = temperature,
        //graphics_clock = graphics_clock,
        percentage_clock_used = percentage_clock_used,
        percentage_mem_used = percentage_mem_used
        //used_mem = used_mem,
        //total_mem = total_mem,
    );
    Ok(())
}
