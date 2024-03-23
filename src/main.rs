use sysinfo::{Disks, Networks, System};
use std::thread;
use std::time::Duration;

fn main() {
    println!("System profiler Version 0.0.1\nApostolos Chalis 2024\n");

    let mut sys = System::new_all();

    loop {
        sys.refresh_all();

        // Clear the terminal
        print!("\x1B[2J\x1B[1;1H");

        // RAM usage
        println!("==> RAM usage.");
        println!("Used memory : {} bytes / {} bytes", sys.used_memory(), sys.total_memory());
        print!("\n");

        // CPU usage
        println!("==> CPU usage.");
        sys.refresh_cpu();
        let mut counter = 0;
        for cpu in sys.cpus() {
            counter += 1;
            print!("CPU {}: {}% ", counter, cpu.cpu_usage());
        }
        print!("\n");

        // Network usage
        println!("\n==> Network usage.");
        let networks = Networks::new_with_refreshed_list();
        for (interface_name, data) in &networks {
            println!(
                "{interface_name}: {} B (down) / {} B (up)",
                data.total_received(),
                data.total_transmitted(),
            );
        }
        print!("\n");

        // Disks usage
        println!("==> Disks usage.");
        let disks = Disks::new_with_refreshed_list();
        for disk in &disks {
            println!("{disk:?}");
        }

        // Sleep for a while before refreshing
        thread::sleep(Duration::from_secs(1)); // Adjust refresh rate as needed
    }
}

