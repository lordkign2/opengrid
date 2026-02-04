use uniffi_bindgen::parse_udl;

fn main() {
    let udl_file = "src/opengrid.udl";
    println!("Parsing UDL file: {}", udl_file);
    
    match parse_udl(udl_file) {
        Ok(interface) => {
            println!("Successfully parsed UDL!");
            println!("Interface name: {:?}", interface.name);
            println!("Functions: {:?}", interface.functions.len());
            for func in &interface.functions {
                println!("  - {}", func.name);
            }
        }
        Err(e) => {
            println!("Failed to parse UDL: {:?}", e);
        }
    }
}