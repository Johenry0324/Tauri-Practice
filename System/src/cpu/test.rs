// Test module - only compiled when running tests
#[cfg(test)]
mod tests {
    use crate::cpu::*;

    #[tokio::test]
    async fn test_get_all_cpu_info_wmi() {
        let all_cpu_info = get_all_cpu_info_wmi().await.expect("Failed to get all CPU info");
        assert!(!all_cpu_info.is_empty(), "Should find at least one processor");
        println!("{:?}", all_cpu_info);
    }
}