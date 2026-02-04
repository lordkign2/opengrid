use std::process::Command;
use std::path::Path;

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let root = Path::new(&manifest_dir).parent().unwrap();
    
    // Paths
    let android_out = root.join("android/app/src/main/java/com/opengrid/generated");
    let ios_out = root.join("ios/Sources/OpenGridIOS/Generated");
    
    // Ensure directories exist (or clean them)
    // Note: diplomat-tool might create them, but good to be safe.
    if !android_out.exists() {
        std::fs::create_dir_all(&android_out).expect("Failed to create android dir");
    }
    if !ios_out.exists() {
        std::fs::create_dir_all(&ios_out).expect("Failed to create ios dir");
    }

    println!("Generating Android bindings...");
    let status_kt = Command::new("diplomat-tool")
        .arg("kotlin")
        .arg(&android_out)
        .current_dir(Path::new(&manifest_dir))
        .status()
        .expect("Failed to run diplomat-tool. Is it installed?");

    if !status_kt.success() {
        panic!("Kotlin binding generation failed");
    }

    println!("Generating iOS bindings...");
    let status_swift = Command::new("diplomat-tool")
        .arg("swift")
        .arg(&ios_out)
        .current_dir(Path::new(&manifest_dir))
        .status()
        .expect("Failed to run diplomat-tool. Is it installed?");

    if !status_swift.success() {
        panic!("Swift binding generation failed");
    }
    
    println!("Bindings generated successfully!");
}
