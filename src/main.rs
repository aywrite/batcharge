// Port of the python batcharge script to rust
use std::process::Command;
use std::io::{self, Write};

fn main() {
    let output = Command::new("ioreg")
        .arg("-rc")
        .arg("AppleSmartBattery")
        .output()
        .expect("failed to get battery status");
    let bat_info = String::from_utf8_lossy(&output.stdout);

    let max_cap: f32 = bat_info
        .lines()
        .find(|x| x.contains("MaxCapacity"))
        .unwrap()
        .chars()
        .skip_while(|c| !c.is_digit(10))
        .collect::<String>()
        .parse()
        .unwrap();
    let cur_cap: f32 = bat_info
        .lines()
        .find(|x| x.contains("CurrentCapacity"))
        .unwrap()
        .chars()
        .skip_while(|c| !c.is_digit(10))
        .collect::<String>()
        .parse()
        .unwrap();

    let charge = cur_cap / max_cap;
    let charge_threshold = (10.0 * charge) as usize;

    let total_slots = 10;
    let filled = charge_threshold * (total_slots / 10);
    let empty = total_slots - filled;

    let color_reset = "%{[00m%}";
    let color = match filled {
        0...3 => "%{[31m%}", // red
        4...6 => "%{[33m%}", // yellow
        _ => "%{[32m%}",     // green
    };

    let out = format!(
        "{}{}{}{}",
        color,
        "◼".repeat(filled),
        "◻".repeat(empty),
        color_reset
    );
    io::stdout().write(out.as_bytes()).ok();
}
