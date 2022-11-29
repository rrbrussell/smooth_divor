use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::process::ExitCode;

use smooth_divor::RPMLead;
use smooth_divor::RPM_LEAD_SIZE;

#[derive(Debug, Parser)]
struct Arguments {
    #[arg(index = 1, value_name = "DIR", value_hint = clap::ValueHint::DirPath)]
    rpm_file: Option<std::path::PathBuf>,
}

fn main() -> ExitCode {
    let arguments = Arguments::parse();
    println!("{arguments:?}");

    let Some(rpm_file) = arguments.rpm_file else {
        println!("Please provide a rpm file to read.");
        return ExitCode::FAILURE;
    };

    let Ok(mut rpm_file) = File::open(rpm_file) else {
        println!("Unable to read the rpm file.");
        return ExitCode::FAILURE;
    };

    let mut buffer = [0; RPM_LEAD_SIZE];
    let Ok(read_bytes) = rpm_file.read(&mut buffer[..]) else {
        println!("I got an error reading the rpm file.");
        return  ExitCode::FAILURE;
    };
    if read_bytes != RPM_LEAD_SIZE {
        println!("I could not read the full lead segment.");
        return ExitCode::FAILURE;
    }

    for base in 0..6 {
        print!("\n{base:04X}:\t");
        for inside in 0..16 {
            if inside > 1 && inside % 2 == 0 {
                print!(" ");
            }
            print!("{:02X}", buffer[base * 16 + inside]);
        }
    }
    print!("\n");

    let Ok(lead) = RPMLead::from_u8_buffer(&buffer) else {
        println!("Unable to parse the lead portion of the RPM file.");
        return ExitCode::FAILURE;
    };

    println!("Valid RPM File: {}", lead.is_valid());
    println!("The package name is: {}", lead.name());

    return ExitCode::SUCCESS;
}
