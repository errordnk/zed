# Building Zed Terminal on Windows

Complete guide for building Zed Terminal fork on Windows.

## Prerequisites

### 1. Install Visual Studio 2022

Download and install [Visual Studio 2022 Community](https://visualstudio.microsoft.com/downloads/)

**Required workloads:**
- ✅ Desktop development with C++

**Required components:**
- Windows 10/11 SDK
- MSVC v143 build tools
- C++ CMake tools for Windows

### 2. Install Rust

Download from [rustup.rs](https://rustup.rs/)

```powershell
# Install Rust (run in PowerShell)
winget install Rustlang.Rustup

# Verify installation
rustc --version
cargo --version

# Add MSVC target (should be default on Windows)
rustup target add x86_64-pc-windows-msvc
```

### 3. Install Node.js

Download from [nodejs.org](https://nodejs.org/) (LTS version recommended)

```powershell
# Verify installation
node --version
npm --version
```

### 4. Install Git

Download from [git-scm.com](https://git-scm.com/download/win)

```powershell
git --version
```

## Clone Repository

```powershell
# Clone your fork
git clone https://github.com/errordnk/zed.git zed-terminal
cd zed-terminal

# Checkout the branch with changes
git checkout main
```

## Build Process

### Option 1: Debug Build (Fast, for development)

```powershell
# Build all workspace members
cargo build

# Binary location:
# target\debug\zed.exe
```

### Option 2: Release Build (Optimized, slower to build)

```powershell
# Build with optimizations
cargo build --release

# Binary location:
# target\release\zed.exe
```

### Option 3: Build specific crate only

```powershell
# Build just the main Zed executable
cargo build --package zed --release

# Build terminal components
cargo build --package terminal_view --release
```

## Running

```powershell
# Run debug build
.\target\debug\zed.exe

# Run release build
.\target\release\zed.exe

# Or use cargo run
cargo run --release
```

## Common Build Issues

### Issue 1: Missing MSVC tools

**Error:** `link.exe not found`

**Solution:**
```powershell
# Install Visual Studio C++ build tools
# Or set environment variable to point to VS installation
$env:PATH += ";C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\<version>\bin\Hostx64\x64"
```

### Issue 2: Out of memory during compilation

**Error:** `fatal error: out of memory allocating X bytes`

**Solution:**
```powershell
# Limit parallel compilation jobs
cargo build --release -j 4

# Or set environment variable permanently
[System.Environment]::SetEnvironmentVariable('CARGO_BUILD_JOBS', '4', 'User')
```

### Issue 3: Long path names

**Error:** `file name too long`

**Solution:**
```powershell
# Enable long paths in Windows (requires admin)
New-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\FileSystem" -Name "LongPathsEnabled" -Value 1 -PropertyType DWORD -Force

# Or clone to shorter path
cd C:\
git clone https://github.com/errordnk/zed.git z
cd z
```

### Issue 4: Antivirus blocking cargo

**Solution:**
- Add exclusion for `target\` directory in Windows Defender
- Add exclusion for `cargo` process

### Issue 5: OpenSSL not found

**Error:** `Could not find OpenSSL`

**Solution:**
```powershell
# Install via vcpkg
git clone https://github.com/Microsoft/vcpkg.git
cd vcpkg
.\bootstrap-vcpkg.bat
.\vcpkg integrate install
.\vcpkg install openssl:x64-windows

# Set environment variable
$env:OPENSSL_DIR = "C:\path\to\vcpkg\installed\x64-windows"
```

## Build Configuration

### Optimize for faster builds

Create `.cargo\config.toml`:

```toml
[build]
# Use parallel linker (requires lld)
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

# Faster incremental compilation
incremental = true

[profile.dev]
# Faster debug builds
opt-level = 1

[profile.release]
# Full optimizations
opt-level = 3
lto = "thin"
codegen-units = 1
```

### Use sccache for faster rebuilds

```powershell
# Install sccache
cargo install sccache

# Configure cargo to use it
$env:RUSTC_WRAPPER = "sccache"

# Or set permanently
[System.Environment]::SetEnvironmentVariable('RUSTC_WRAPPER', 'sccache', 'User')
```

## Clean Build

```powershell
# Remove all build artifacts
cargo clean

# Remove only release artifacts
cargo clean --release

# Full clean including dependencies
Remove-Item -Recurse -Force target\
```

## Build Time Estimates

**First build (no cache):**
- Debug: 30-60 minutes
- Release: 60-120 minutes

**Incremental builds (after code changes):**
- Debug: 1-5 minutes
- Release: 5-15 minutes

**Hardware matters:**
- SSD: 2-3x faster than HDD
- 16GB+ RAM: Recommended
- Multi-core CPU: Faster parallel compilation

## Distribution

### Create portable build

```powershell
# Build release
cargo build --release

# Copy required files
mkdir zed-terminal-portable
copy target\release\zed.exe zed-terminal-portable\
copy -Recurse assets zed-terminal-portable\
copy -Recurse crates\zed\resources zed-terminal-portable\

# Create archive
Compress-Archive -Path zed-terminal-portable -DestinationPath zed-terminal-windows.zip
```

## Development Tips

### Fast edit-compile-test cycle

```powershell
# Use cargo watch for auto-rebuild
cargo install cargo-watch

# Watch and rebuild on changes
cargo watch -x "build --package zed"

# Watch and run on changes
cargo watch -x "run --package zed"
```

### Check without full build

```powershell
# Fast syntax/type checking
cargo check

# Check with all features
cargo check --all-features
```

### Build only what changed

```powershell
# Cargo automatically does incremental builds
# Just run cargo build again after code changes
cargo build
```

## Troubleshooting

### View detailed build errors

```powershell
# Verbose output
cargo build -vv

# Show full backtraces
$env:RUST_BACKTRACE = "full"
cargo build
```

### Check dependencies

```powershell
# List all dependencies
cargo tree

# Find duplicate dependencies
cargo tree --duplicates
```

### Verify Rust installation

```powershell
rustc --version --verbose
cargo --version --verbose
rustup show
```

## Additional Resources

- [Rust on Windows](https://rust-lang.github.io/rustup/installation/windows.html)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Zed Repository](https://github.com/zed-industries/zed)
- [Our Fork](https://github.com/errordnk/zed)

## Next Steps

After successful build:
1. Read [SETTINGS.md](SETTINGS.md) for configuration
2. Set up terminal profiles in settings.json
3. Configure AI providers (Zed Free or API keys)
4. Customize keybindings and themes
