#!/usr/bin/env python3
"""
Validation script for OpenGrid Core workspace structure
"""

import os
import sys
from pathlib import Path

def validate_structure():
    """Validate that the workspace structure matches requirements"""
    
    root = Path(".")
    
    # Required files and directories
    required_paths = [
        "Cargo.toml",
        "README.md",
        "core/Cargo.toml",
        "core/src/lib.rs",
        "core/src/node.rs",
        "core/src/ledger.rs",
        "core/src/crdt/mod.rs",
        "core/src/crdt/traits.rs",
        "core/src/sync/mod.rs",
        "core/src/sync/protocol.rs",
        "core/src/crypto/mod.rs",
        "core/src/storage/mod.rs",
        "core/src/error.rs",
        "ffi/Cargo.toml",
        "ffi/src/lib.rs",
    ]
    
    print("🔍 Validating OpenGrid workspace structure...")
    print("=" * 50)
    
    missing = []
    for path_str in required_paths:
        path = root / path_str
        if not path.exists():
            missing.append(path_str)
            print(f"❌ Missing: {path_str}")
        else:
            print(f"✅ Found: {path_str}")
    
    print("=" * 50)
    
    if missing:
        print(f"\n🚨 {len(missing)} required files are missing:")
        for path in missing:
            print(f"   - {path}")
        return False
    else:
        print("🎉 All required files are present!")
        return True

def validate_content():
    """Validate key content requirements"""
    
    print("\n🔍 Validating content requirements...")
    print("=" * 50)
    
    checks = []
    
    # Check Cargo.toml workspace definition
    cargo_toml = Path("Cargo.toml")
    if cargo_toml.exists():
        content = cargo_toml.read_text()
        has_workspace = "[workspace]" in content
        has_members = 'members = [' in content and '"core"' in content and '"ffi"' in content
        checks.append(("Workspace definition", has_workspace and has_members))
    
    # Check core lib.rs exports
    lib_rs = Path("core/src/lib.rs")
    if lib_rs.exists():
        content = lib_rs.read_text()
        has_mod_declarations = all(mod in content for mod in [
            "pub mod node;",
            "pub mod ledger;",
            "pub mod crdt;",
            "pub mod sync;",
            "pub mod crypto;",
            "pub mod storage;",
            "pub mod error;"
        ])
        checks.append(("Module declarations", has_mod_declarations))
    
    # Check FFI lib.rs has UniFFI setup
    ffi_lib_rs = Path("ffi/src/lib.rs")
    if ffi_lib_rs.exists():
        content = ffi_lib_rs.read_text()
        has_uniffi_setup = "uniffi::setup_scaffolding!" in content
        has_exports = "#[uniffi::export]" in content
        checks.append(("UniFFI setup", has_uniffi_setup and has_exports))
    
    # Check for tests in Rust files
    rust_files = list(Path(".").rglob("*.rs"))
    has_tests = any("#[cfg(test)]" in f.read_text() for f in rust_files)
    checks.append(("Test modules present", has_tests))
    
    # Print results
    all_passed = True
    for check_name, passed in checks:
        status = "✅" if passed else "❌"
        print(f"{status} {check_name}")
        if not passed:
            all_passed = False
    
    print("=" * 50)
    return all_passed

def main():
    """Main validation function"""
    
    print("OpenGrid Core Structure Validator")
    print("=" * 50)
    
    structure_valid = validate_structure()
    content_valid = validate_content() if structure_valid else False
    
    print("\n📋 SUMMARY")
    print("=" * 50)
    
    if structure_valid and content_valid:
        print("🎉 Validation PASSED!")
        print("The OpenGrid workspace structure is complete and valid.")
        print("\nNext steps:")
        print("1. Install Rust: https://www.rust-lang.org/tools/install")
        print("2. Run: cargo build")
        print("3. Run: cargo test")
        return 0
    else:
        print("❌ Validation FAILED!")
        print("Please fix the issues above before proceeding.")
        return 1

if __name__ == "__main__":
    sys.exit(main())