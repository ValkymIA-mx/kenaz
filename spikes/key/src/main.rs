use std::{error::Error, process::ExitCode};
use yubikit::{
    core::Transport,
    ctap::CtapSession,
    ctap2::Ctap2Session,
    device::{get_name, read_info_ccid},
    oath::OathSession,
    platform::{
        hidapi::{list_fido_devices, list_otp_devices, HidFidoConnection},
        pcsc::{is_reader_usb, list_readers, PcscSmartCardConnection},
    },
};

// Never call list_devices/open_smartcard here: their automatic recovery can kill
// scdaemon without asking. Direct shared connections preserve the original error.
fn probe_reader(reader: &str) -> Result<(), Box<dyn Error>> {
    let connection = PcscSmartCardConnection::new(reader, false)?;
    let (info, connection) = read_info_ccid(connection, None)?;
    println!("model={} serial={:?} firmware={} form_factor={}",
        get_name(&info), info.serial, info.version, info.form_factor);
    for transport in [Transport::Usb, Transport::Nfc] {
        println!("{transport:?}: supported={:?} enabled={:?}",
            info.supported_capabilities.get(&transport),
            info.config.enabled_capabilities.get(&transport));
    }
    let mut oath = OathSession::new(connection).map_err(|(error, _)| error)?;
    // Account labels are intentionally not logged. No PIN attempts, writes,
    // calculations (which could advance HOTP), or reset commands are performed.
    println!("OATH credential count={}", oath.list_credentials()?.len());
    Ok(())
}

fn main() -> ExitCode {
    let probe = match std::env::args().nth(1).as_deref() {
        None => false,
        Some("--probe") => true,
        _ => {
            eprintln!("Usage: kenaz-key-spike [--probe]\nDiscovery only by default; --probe reads device info, OATH count and CTAP2 getInfo. No writes or PIN attempts.");
            return ExitCode::from(64);
        }
    };
    let mut found = false;
    let mut failed = false;
    match list_readers() {
        Ok(readers) => {
            println!("PC/SC readers={}", readers.len());
            for reader in readers.iter().filter(|reader| is_reader_usb(reader)) {
                found = true;
                println!("Yubico USB reader: {reader}");
                if probe {
                    if let Err(error) = probe_reader(reader) {
                        eprintln!("PC/SC probe: {error:?}");
                        failed = true;
                    }
                }
            }
        }
        Err(error) => {
            eprintln!("PC/SC enumeration: {error:?} ({error})");
            failed = true;
        }
    }
    match list_fido_devices() {
        Ok(devices) => {
            println!("FIDO HID devices={}", devices.len());
            for device in devices {
                found = true;
                println!("FIDO PID={:04x} USB firmware={}", device.pid, device.version);
                if probe {
                    let result = (|| -> Result<(), Box<dyn Error>> {
                        let connection = HidFidoConnection::open(&device)?;
                        let session = CtapSession::new_fido(connection).map_err(|(e, _)| e)?;
                        let mut ctap = Ctap2Session::new(session).map_err(|(e, _)| e)?;
                        println!("CTAP2 getInfo={:?}", ctap.get_info()?);
                        Ok(())
                    })();
                    if let Err(error) = result {
                        eprintln!("FIDO probe: {error:?}");
                        failed = true;
                    }
                }
            }
        }
        Err(error) => {
            eprintln!("FIDO HID enumeration: {error:?} ({error})");
            failed = true;
        }
    }
    match list_otp_devices() {
        Ok(devices) => {
            println!("OTP HID devices={}", devices.len());
            found |= !devices.is_empty();
        }
        Err(error) => {
            eprintln!("OTP HID enumeration: {error:?} ({error})");
            failed = true;
        }
    }
    if failed {
        ExitCode::FAILURE
    } else if !found {
        eprintln!("No YubiKey discovered; hardware acceptance has NOT run.");
        ExitCode::from(2)
    } else {
        ExitCode::SUCCESS
    }
}
