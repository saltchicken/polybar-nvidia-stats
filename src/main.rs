use nvml_wrapper::enum_wrappers::device::{Clock, TemperatureSensor};
use nvml_wrapper::error::NvmlError;
//use nvml_wrapper::{cuda_driver_version_major, cuda_driver_version_minor, Nvml};
use nvml_wrapper::{Nvml};
//use pretty_bytes::converter::convert;

fn clock_tier(percentage: u32) -> String {
    if percentage < 30 {
        format!("%{{F#00FF00}}{}%{{F-}}", percentage) // Green for low percentages
    } else if percentage < 70 {
        format!("%{{F#FFFF00}}{}%{{F-}}", percentage) // Yellow for medium percentages
    } else {
        format!("%{{F#FF0000}}{}%{{F-}}", percentage) // Red for high percentages
    }
}

fn memory_tier(percentage: u64) -> String {
    if percentage < 30 {
        format!("%{{F#00FF00}}{}%{{F-}}", percentage) // Green for low percentages
    } else if percentage < 70 {
        format!("%{{F#FFFF00}}{}%{{F-}}", percentage) // Yellow for medium percentages
    } else {
        format!("%{{F#FF0000}}{}%{{F-}}", percentage) // Red for high percentages
    }
}

fn temperature_tier(temperature: u32) -> String {
    if temperature <= 45 {
        format!("%{{F#00FF00}}{}%{{F-}}", temperature) // Green for low percentages
    } else if temperature <= 60 {
        format!("%{{F#FFFF00}}{}%{{F-}}", temperature) // Yellow for medium percentages
    } else {
        format!("%{{F#FF0000}}{}%{{F-}}", temperature) // Red for high percentages
    }
}

fn main() -> Result<(), NvmlError> {
    let nvml = Nvml::init()?;

    //let cuda_version = nvml.sys_cuda_driver_version()?;

    // Grabbing the first device in the system, whichever one that is.
    // If you want to ensure you get the same physical device across reboots,
    // get devices via UUID or PCI bus IDs.
    let device = nvml.device_by_index(0)?;

    let temperature = device.temperature(TemperatureSensor::Gpu)?;
    let formatted_temperature = temperature_tier(temperature);

    let percentage_clock_used = device.clock_info(Clock::Graphics)? / device.max_clock_info(Clock::Graphics)?;
    let formatted_percentage_clock_used = clock_tier(percentage_clock_used);

    let mem_info = device.memory_info()?;
    let percentage_mem_used = mem_info.used / mem_info.total;
    let formatted_percentage_mem_used = memory_tier(percentage_mem_used);

    println!(
        //"{graphics_clock} {used_mem}/{total_mem} {temperature}",
        //"{percentage_clock_used} {percentage_mem_used} %{{F#FFFF00}}{temperature}%{{F-}}",
        "{percentage_clock_used} {percentage_mem_used} {temperature}",
        //graphics_clock = graphics_clock,
        percentage_clock_used = formatted_percentage_clock_used,
        percentage_mem_used = formatted_percentage_mem_used,
        temperature = formatted_temperature,
        //used_mem = used_mem,
        //total_mem = total_mem,
    );
    Ok(())
}
