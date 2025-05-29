mod test;

use wmi::{COMLibrary, WMIConnection};
use serde::Deserialize;
use anyhow::{Result, Context};

#[derive(Deserialize, Debug, Clone)]
pub struct Win32Processor {
    #[serde(rename = "AddressWidth")]
    pub address_width: Option<u16>,
    #[serde(rename = "Architecture")]
    pub architecture: Option<u16>,
    #[serde(rename = "AssetTag")]
    pub asset_tag: Option<String>,
    #[serde(rename = "Availability")]
    pub availability: Option<u16>,
    #[serde(rename = "Caption")]
    pub caption: Option<String>,
    #[serde(rename = "Characteristics")]
    pub characteristics: Option<u32>,
    #[serde(rename = "ConfigManagerErrorCode")]
    pub config_manager_error_code: Option<u32>,
    #[serde(rename = "ConfigManagerUserConfig")]
    pub config_manager_user_config: Option<bool>,
    #[serde(rename = "CpuStatus")]
    pub cpu_status: Option<u16>,
    #[serde(rename = "CreationClassName")]
    pub creation_class_name: Option<String>,
    #[serde(rename = "CurrentClockSpeed")]
    pub current_clock_speed: Option<u32>,
    #[serde(rename = "CurrentVoltage")]
    pub current_voltage: Option<u16>,
    #[serde(rename = "DataWidth")]
    pub data_width: Option<u16>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "DeviceID")]
    pub device_id: Option<String>,
    #[serde(rename = "ErrorCleared")]
    pub error_cleared: Option<bool>,
    #[serde(rename = "ErrorDescription")]
    pub error_description: Option<String>,
    #[serde(rename = "ExtClock")]
    pub ext_clock: Option<u32>,
    #[serde(rename = "Family")]
    pub family: Option<u16>,
    #[serde(rename = "InstallDate")]
    pub install_date: Option<String>,
    #[serde(rename = "L2CacheSize")]
    pub l2_cache_size: Option<u32>,
    #[serde(rename = "L2CacheSpeed")]
    pub l2_cache_speed: Option<u32>,
    #[serde(rename = "L3CacheSize")]
    pub l3_cache_size: Option<u32>,
    #[serde(rename = "L3CacheSpeed")]
    pub l3_cache_speed: Option<u32>,
    #[serde(rename = "LastErrorCode")]
    pub last_error_code: Option<u32>,
    #[serde(rename = "Level")]
    pub level: Option<u16>,
    #[serde(rename = "LoadPercentage")]
    pub load_percentage: Option<u16>,
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,
    #[serde(rename = "MaxClockSpeed")]
    pub max_clock_speed: Option<u32>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "NumberOfCores")]
    pub number_of_cores: Option<u32>,
    #[serde(rename = "NumberOfEnabledCore")]
    pub number_of_enabled_core: Option<u32>,
    #[serde(rename = "NumberOfLogicalProcessors")]
    pub number_of_logical_processors: Option<u32>,
    #[serde(rename = "OtherFamilyDescription")]
    pub other_family_description: Option<String>,
    #[serde(rename = "PartNumber")]
    pub part_number: Option<String>,
    #[serde(rename = "PNPDeviceID")]
    pub pnp_device_id: Option<String>,
    #[serde(rename = "PowerManagementCapabilities")]
    pub power_management_capabilities: Option<Vec<u16>>,
    #[serde(rename = "PowerManagementSupported")]
    pub power_management_supported: Option<bool>,
    #[serde(rename = "ProcessorId")]
    pub processor_id: Option<String>,
    #[serde(rename = "ProcessorType")]
    pub processor_type: Option<u16>,
    #[serde(rename = "Revision")]
    pub revision: Option<u16>,
    #[serde(rename = "Role")]
    pub role: Option<String>,
    #[serde(rename = "SecondLevelAddressTranslationExtensions")]
    pub second_level_address_translation_extensions: Option<bool>,
    #[serde(rename = "SerialNumber")]
    pub serial_number: Option<String>,
    #[serde(rename = "SocketDesignation")]
    pub socket_designation: Option<String>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "StatusInfo")]
    pub status_info: Option<u16>,
    #[serde(rename = "Stepping")]
    pub stepping: Option<String>,
    #[serde(rename = "SystemCreationClassName")]
    pub system_creation_class_name: Option<String>,
    #[serde(rename = "SystemName")]
    pub system_name: Option<String>,
    #[serde(rename = "ThreadCount")]
    pub thread_count: Option<u32>,
    #[serde(rename = "UniqueId")]
    pub unique_id: Option<String>,
    #[serde(rename = "UpgradeMethod")]
    pub upgrade_method: Option<u16>,
    #[serde(rename = "Version")]
    pub version: Option<String>,
    #[serde(rename = "VirtualizationFirmwareEnabled")]
    pub virtualization_firmware_enabled: Option<bool>,
    #[serde(rename = "VMMonitorModeExtensions")]
    pub vm_monitor_mode_extensions: Option<bool>,
    #[serde(rename = "VoltageCaps")]
    pub voltage_caps: Option<u32>,
}

pub async fn get_all_cpu_info_wmi() -> Result<Vec<Win32Processor>> {
    let com_con = COMLibrary::new()
        .context("Failed to initialize COM library")?;
    let wmi_con = WMIConnection::new(com_con.into())
        .context("Failed to establish WMI connection")?;
    let results: Vec<Win32Processor> = wmi_con
        .raw_query("SELECT * FROM Win32_Processor")
        .context("Failed to query Win32_Processor information")?;

    if results.is_empty() {
        return Err(anyhow::anyhow!("No processors found in the system"));
    }

    Ok(results)
}