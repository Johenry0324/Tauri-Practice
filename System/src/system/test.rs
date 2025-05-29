#[cfg(test)]

mod tests {
    use crate::system;

    #[test]
    fn test_get_cpu_info() {
        system::get_cpu_info();
    }
}
