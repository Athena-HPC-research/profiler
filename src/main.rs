// Author: Apostolos Chalis 2024
// Rust CLI system profiler
use sysinfo::{
    Disks, Networks, System,
};

fn main() {
    println!("System profiler Version 0.0.1\nApostolos Chalis 2024\n");

    let mut sys = System::new_all();
    sys.refresh_all();
    
    // RAM usage
    println!("==> RAM usage.");
    println!("Used memory : {} bytes / {} bytes", sys.used_memory(), sys.total_memory());
    print!("\n");

    // CPU usage
    println!("==> CPU usage.");
    sys.refresh_cpu();
    let mut counter = 0;

    for cpu in sys.cpus() {
        counter+= 1;
        print!("CPU {}: {}% ",counter, cpu.cpu_usage());
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
}
