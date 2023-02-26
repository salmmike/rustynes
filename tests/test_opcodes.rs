
#[cfg(test)]
mod tests {
    use rustyneslib::{CPU,Bus};

    #[test]
    pub fn test_cpu() {
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.tick(&mut bus);
    }
}