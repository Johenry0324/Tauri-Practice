use wmi::{COMLibrary, WMIConnection};
use anyhow::{Result, Context};
use serde::Deserialize;
use nvml_wrapper::{struct_wrappers::device::ProcessUtilizationSample, Nvml};

#[derive(Deserialize, Debug, Clone)]
pub struct Win32VideoController {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "AdapterRAM")]
    pub adapter_ram: Option<u64>,
    #[serde(rename = "DriverVersion")]
    pub driver_version: Option<String>,
    #[serde(rename = "PNPDeviceID")]
    pub pnp_device_id: Option<String>,
    #[serde(rename = "VideoProcessor")]
    pub video_processor: Option<String>,
    #[serde(rename = "CurrentRefreshRate")]
    pub current_refresh_rate: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct GPUInfo {
    pub dev_name: String,
    pub total_memory: u64,
    pub used_memory: u64,
    pub uuid: String,
}

pub async fn get_all_gpu_info_wmi() -> Result<Vec<Win32VideoController>> {
    let com_con = COMLibrary::new()?;
    let wmi_con = WMIConnection::new(com_con.into())?;

    let results: Vec<Win32VideoController> = wmi_con
        .raw_query("SELECT * FROM Win32_VideoController")
        .context("Failed to query Win32_VideoController information")?;

    Ok(results)
}      

pub async fn get_all_gpu_info_nvml() -> Result<Vec<GPUInfo>> {
    let nvml = Nvml::init()?;
    let devices_count = nvml.device_count()?;
    let mut results = Vec::new();
    for i in 0..devices_count {
        let device = nvml.device_by_index(i)?;
        let name = device.name()?;
        let total_memory = device.memory_info()?.total;
        let used_memory = device.memory_info()?.used;
        let uuid = device.uuid()?;
            results.push(GPUInfo {
            dev_name: name,
            total_memory,
            used_memory,
            uuid,
        });
    }

    Ok(results)
}

pub fn get_all_process_gpu_utilization_nvml() -> Result<Vec<ProcessUtilizationSample>> {
    let nvml = Nvml::init()?;
    let device_count = nvml.device_count()?;
    let mut all_samples = Vec::new();

    for i in 0..device_count {
        let device = nvml.device_by_index(i)?;
        let utilization = device.utilization_rates()?;
        println!("GPU {:?} utilization: {:?}", device.name()?, utilization);
    }

    Ok(all_samples)
}

#[cfg(test)]
mod tests {
    use super::*;

    /* 
    #[tokio::test]
    async fn test_get_all_gpu_info_wmi() {
        let all_gpu_info = get_all_gpu_info_wmi().await.expect("Failed to get all GPU info");
        assert!(!all_gpu_info.is_empty(), "Should find at least one GPU");
        println!("GPU {:?}", all_gpu_info);
    }
    */

    #[tokio::test]
    async fn test_get_all_gpu_info_nvml() {
        let all_gpu_info = get_all_gpu_info_nvml().await.expect("Failed to get all GPU info");
        assert!(!all_gpu_info.is_empty(), "Should find at least one GPU");
        println!("GPU {:?}", all_gpu_info);
    }

    #[test]
    fn test_get_all_process_gpu_utilization_nvml() {
        let samples = get_all_process_gpu_utilization_nvml();
    }
}
