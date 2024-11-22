use std::thread;
use std::time::Duration;

struct Pwm {
    duty_cycle: f32, // Duty cycle as a percentage (0.0 to 1.0)
    frequency: u32,  // Frequency in Hz
}

impl Pwm {
    fn new(duty_cycle: f32, frequency: u32) -> Self {
        Pwm { duty_cycle, frequency }
    }

    fn start(&self) {
        let period = Duration::from_micros((1_000_000 / self.frequency) as u64);
        let high_time = period.mul_f32(self.duty_cycle);
        let low_time = period - high_time;

        loop {
            // Simulate high signal
            println!("HIGH");
            thread::sleep(high_time);

            // Simulate low signal
            println!("LOW");
            thread::sleep(low_time);

            // Add delay to make the output more readable
            thread::sleep(Duration::from_millis(100)); // 100 ms delay
        }
    }
}

fn main() {
    let pwm = Pwm::new(0.5, 1); // 50% duty cycle, 1 Hz frequency
    pwm.start();
}
