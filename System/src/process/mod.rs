mod test;

use wmi::{COMLibrary, WMIConnection};
use serde::Deserialize;
use anyhow::{Result, Context};

#[derive(Deserialize, Debug, Clone)]
pub struct Win32PerfRawDataPerfProcProcess  {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "PercentProcessorTime")]
    pub percent_processor_time: u64,
    #[serde(rename = "IDProcess")]
    pub process_id: u32,
    #[serde(rename = "Timestamp_Sys100NS")]
    pub timestamp: u64,
}

pub async fn get_all_process_info_wmi() -> Result<Vec<Win32PerfRawDataPerfProcProcess>> {
    let com_con = COMLibrary::new()?;
    let wmi_con = WMIConnection::new(com_con.into())?;

    let results: Vec<Win32PerfRawDataPerfProcProcess> = wmi_con
        .raw_query("SELECT Name, PercentProcessorTime, IDProcess, Timestamp_Sys100NS FROM Win32_PerfRawData_PerfProc_Process")
        .context("Failed to query Win32_Process information")?;

    if results.is_empty() {
        return Err(anyhow::anyhow!("No processes found in the system"));
    }   

    Ok(results)
}