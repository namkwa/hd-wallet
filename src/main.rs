use std::str::FromStr;

fn main() {
    use tss_esapi::{Context, TctiNameConf};

    // Specify the TCTI (TPM Command Transmission Interface) name for the TPM device
    let tcti_name = "device:/dev/tpmrm0";

    // Create a new TPM context using the specified TCTI
    let tcti = TctiNameConf::from_str(tcti_name).unwrap();
    let _context = Context::new(tcti).unwrap();
}
