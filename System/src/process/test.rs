
#[cfg(test)]
mod tests {
    use crate::cpu::*;
    use crate::process::*;

    #[tokio::test]
    async fn test_get_all_process_info_wmi() {
        let process_info = get_all_process_info_wmi().await.expect("Failed to get process info");
        assert!(!process_info.is_empty(), "Should find at least one process");
    }

    #[tokio::test]
    async fn test_get_process_cpu_usage_wmi() {
        let cpu_info = get_all_cpu_info_wmi().await.expect("Failed to get cpu info");
        let core_count = cpu_info[0].number_of_logical_processors.unwrap_or(1);

        let first_snapshot = get_all_process_info_wmi().await.expect("Failed to get process info");
        std::thread::sleep(std::time::Duration::from_secs(1));
        let second_snapshot = get_all_process_info_wmi().await.expect("Failed to get process info");

        let skip_names = ["_Total", "Idle", "System", "System Idle Process"];
        let mut usages: Vec<(String, u32, f64)> = Vec::new();

        for second in &second_snapshot {
            if skip_names.contains(&second.name.as_str()) || second.process_id == 0 {
                continue;
            }
            if let Some(first) = first_snapshot.iter().find(|p| p.process_id == second.process_id) {
                let delta_cpu = second.percent_processor_time.saturating_sub(first.percent_processor_time);
                let delta_time = second.timestamp.saturating_sub(first.timestamp);
                if delta_time > 0 && core_count > 0 {
                    let usage = 100.0 * (delta_cpu as f64) / (delta_time as f64 * core_count as f64);
                    if usage >= 0.0 && usage <= 100.0 {
                        usages.push((second.name.clone(), second.process_id, usage));
                    }
                }
            }
        }

        usages.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

        for (name, pid, usage) in usages {
            if usage > 0.0 {    
                println!("Process {} (PID {}): CPU usage = {:.2}%", name, pid, usage);
            }
        }
    }   
}       