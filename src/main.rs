use clap::Parser;
use serialport::SerialPortType;
use std::cmp::Ordering;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Sort mode: 0 for default, 1 for port number
    #[arg(long, default_value_t = 1)]
    sort: u8,

    /// Show only USB ports
    #[arg(short = 'u', long)]
    usb_only: bool,

    /// Show only PCI ports
    #[arg(long)]
    pci_only: bool,

    /// Show only Bluetooth ports
    #[arg(long)]
    bt_only: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn extract_number(name: &str) -> Option<u32> {
    name.chars()
        .skip_while(|c| !c.is_digit(10))
        .take_while(|c| c.is_digit(10))
        .collect::<String>()
        .parse()
        .ok()
}

fn port_sort(a: &serialport::SerialPortInfo, b: &serialport::SerialPortInfo) -> Ordering {
    let a_num = extract_number(&a.port_name);
    let b_num = extract_number(&b.port_name);
    match (a_num, b_num) {
        (Some(a), Some(b)) => a.cmp(&b),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => a.port_name.cmp(&b.port_name),
    }
}

fn main() {
    let args = Args::parse();

    match serialport::available_ports() {
        Ok(mut ports) => {
            match args.sort {
                1 => ports.sort_by(port_sort),
                _ => {} // 默认排序，不做任何操作
            }

            let filtered_ports: Vec<_> = ports
                .into_iter()
                .filter(|p| {
                    (!args.usb_only && !args.pci_only && !args.bt_only) ||
                        (args.usb_only && matches!(p.port_type, SerialPortType::UsbPort(_))) ||
                        (args.pci_only && matches!(p.port_type, SerialPortType::PciPort)) ||
                        (args.bt_only && matches!(p.port_type, SerialPortType::BluetoothPort))
                })
                .collect();

            println!("Found {} serial ports:", filtered_ports.len());
            for p in filtered_ports {
                let port_info = match p.port_type {
                    SerialPortType::UsbPort(info) => {
                        if args.verbose {
                            format!("[USB] ID: {:04x}/{:04x}, SN: {}, MF: {}, Name: {}",
                                    info.vid,
                                    info.pid,
                                    info.serial_number.as_deref().unwrap_or("N/A"),
                                    info.manufacturer.as_deref().unwrap_or("N/A"),
                                    info.product.as_deref().unwrap_or("USB Serial Port")
                            )
                        } else {
                            format!("[USB] Name: {}", info.product.as_deref().unwrap_or("USB Serial Port"))
                        }
                    }
                    SerialPortType::BluetoothPort => "[Bluetooth]".to_string(),
                    SerialPortType::PciPort => "[PCI]".to_string(),
                    _ => "[Other]".to_string(),
                };
                println!("{}: {}", p.port_name, port_info);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}