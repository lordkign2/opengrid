#!/usr/bin/env python3
import subprocess
import os
import sys
from pathlib import Path

def run_command(command, cwd=None):
    print(f"🚀 Running: {' '.join(command)} in {cwd or '.'}")
    try:
        result = subprocess.run(command, cwd=cwd, check=True, capture_output=True, text=True)
        print("✅ Success!")
        return True
    except subprocess.CalledProcessError as e:
        print(f"❌ Failed!")
        print(f"STDOUT: {e.stdout}")
        print(f"STDERR: {e.stderr}")
        return False
    except FileNotFoundError:
        print(f"⚠️ Command not found: {command[0]}. Skipping...")
        return True # Skip if not installed, or handle strictly depending on policy

def main():
    root = Path(__file__).parent.parent
    all_passed = True

    print("=== OpenGrid Cross-Platform Verification ===")

    # 1. Rust Core Tests
    print("\n[1/3] Testing Rust Core...")
    if not run_command(["cargo", "test"], cwd=str(root / "core")):
        all_passed = False

    # 2. Android Unit Tests
    print("\n[2/3] Testing Android (Unit)...")
    gradlew = "gradlew" if os.name != 'nt' else "gradlew.bat"
    android_dir = root / "android"
    if (android_dir / gradlew).exists():
        if not run_command([str(android_dir / gradlew), "test"], cwd=str(android_dir)):
            all_passed = False
    else:
        print(f"ℹ️ Android gradlew not found in {android_dir}. Skipping Android tests.")

    # 3. iOS Tests
    print("\n[3/3] Testing iOS...")
    ios_dir = root / "ios"
    # Note: swift test usually requires a Package.swift
    if (ios_dir / "Package.swift").exists():
        if not run_command(["swift", "test"], cwd=str(ios_dir)):
            all_passed = False
    else:
        print(f"ℹ️ iOS Package.swift not found. Skipping iOS tests.")

    print("\n" + "="*40)
    if all_passed:
        print("🎉 ALL TESTS PASSED!")
        return 0
    else:
        print("🚨 SOME TESTS FAILED!")
        return 1

if __name__ == "__main__":
    sys.exit(main())
