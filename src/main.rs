// Author: Apostolos Chalis 2024
use sysinfo::{
    Disks, Networks, System,
};

fn main() {
    println!("System profiler Version 0.0.1\nApostolos Chalis 2024");

    let mut sys = System::new_all();
    sys.refresh_all();

    // RAM usage
    println!("Used memory : {} bytes / {} bytes", sys.used_memory(), sys.total_memory());

    // CPU usage
    sys.refresh_cpu();
    let mut counter = 0;

    for cpu in sys.cpus() {
        counter+= 1;
        print!("CPU {}: {}% ",counter, cpu.cpu_usage());
    }

    let networks = Networks::new_with_refreshed_list();

    print!("\n");

    for (interface_name, data) in &networks {
        println!(
        "{interface_name}: {} B (down) / {} B (up)",
        data.total_received(),
        data.total_transmitted(),
        );
    }

    println!("=> disks:");
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        println!("{disk:?}");
    }
}
