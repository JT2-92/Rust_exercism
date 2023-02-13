use semi_structured_logs::{info, log, LogLevel};

fn main() {
    println!("{}", log(LogLevel::Error, "Stack overflow!"));
    println!("{}", info("Timezone changed"));
}
