use binrw::BinRead;
use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::process::ExitCode;

use smooth_divor::cli_support::print_buffer_as_hex;
use smooth_divor::RPMHeader;
use smooth_divor::RPMLead;
use smooth_divor::RPMSignatureIndexEntry;
use smooth_divor::RPM_HEADER_SIZE;
use smooth_divor::RPM_LEAD_SIZE;

#[derive(Debug, Parser)]
struct Arguments {
    #[arg(index = 1, value_name = "DIR", value_hint = clap::ValueHint::DirPath)]
    rpm_file: Option<std::path::PathBuf>,
}

fn main() -> ExitCode {
    let arguments = Arguments::parse();

    let Some(rpm_file) = arguments.rpm_file else {
        println!("Please provide a rpm file to read.");
        return ExitCode::FAILURE;
    };

    let Ok(mut rpm_file) = File::open(rpm_file) else {
        println!("Unable to read the rpm file.");
        return ExitCode::FAILURE;
    };

    let mut buffer = [0u8; RPM_LEAD_SIZE];
    let Ok(read_bytes) = rpm_file.read(&mut buffer[..]) else {
        println!("I got an error reading the rpm file.");
        return  ExitCode::FAILURE;
    };
    if read_bytes != RPM_LEAD_SIZE {
        println!("I could not read the full lead segment.");
        return ExitCode::FAILURE;
    }

    print_buffer_as_hex(&buffer);
    print!("\n");

    rpm_file.rewind().unwrap();
    match RPMLead::read(&mut rpm_file) {
        Err(e) => {
            println!("Unable to parse the lead portion of the RPM file.");
            println!("{e:#?}");
            return ExitCode::FAILURE;
        }
        Ok(lead) => {
            println!("Valid RPM File: {}", lead.is_valid());
            println!("The package name is: {}", lead.name());
        }
    };

    print!("\n");

    let before_signature: u64 = rpm_file.stream_position().unwrap();

    let mut buffer = [0u8; RPM_HEADER_SIZE];

    match rpm_file.read(&mut buffer[..]) {
        Err(e) => {
            println!("I got an error reading the rpm file.");
            println!("{e:#?}");
            return ExitCode::FAILURE;
        }
        Ok(read_bytes) => {
            if read_bytes != RPM_HEADER_SIZE {
                println!("Could not read the full signature segment");
                return ExitCode::FAILURE;
            }
        }
    }

    print_buffer_as_hex(&buffer);
    print!("\n");

    rpm_file.seek(SeekFrom::Start(before_signature)).unwrap();

    let Ok(signature) = RPMHeader::read(&mut rpm_file) else {
            println!("Unable to parse the signature portion of the RPM file.");
            return ExitCode::FAILURE;
        };

    println!("Valid Signature Header: {}", signature.is_valid());
    println!(
        "There are {} items in the signature.",
        signature.number_of_index_entries()
    );
    println!(
        "The signature is {} bytes in size.",
        signature.header_size()
    );

    let mut signature_entries =
        Vec::<RPMSignatureIndexEntry>::with_capacity(signature.number_of_index_entries() as usize);
    for _ in 0..signature.number_of_index_entries() {
        match RPMSignatureIndexEntry::read(&mut rpm_file) {
            Ok(entry) => {
                signature_entries.push(entry);
            }
            Err(e) => {
                println!("{e:#?} while attempting to read a Signature entry")
            }
        }
    }
    for entry in signature_entries.iter() {
        println!("{entry}");
    }
    return ExitCode::SUCCESS;
}
