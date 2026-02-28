# Anchor - Hardware-Based Security Application

Anchor is a cross-platform desktop application that provides hardware-based authentication using SanDisk USB devices. The application automatically detects USB connection/disconnection events and provides secure database access only when an authorized USB device is connected.

## 🚀 Features

### 🔐 Hardware-Based Security
- **USB Heartbeat**: Real-time detection of SanDisk USB devices (VID: 0x0781)
- **Event-Driven Architecture**: Instant UI updates on USB connection/disconnection
- **Session Wipe**: Immediate application lock when USB is removed
- **Encrypted Database**: SQLite database only accessible when USB is connected

### 🌐 Cross-Platform Support
- **Windows**: WinUSB driver integration via Zadig
- **macOS**: Native IOKit support (no special setup required)
- **Linux**: udev rule generation for non-root USB access

### 🛡️ Security Features
- **Thread-Safe State**: Rust-based backend with Arc<Mutex<AppState>>
- **No Persistent Credentials**: Authentication tied to physical USB device
- **Immediate Lockout**: Session terminates instantly on USB removal
- **Device-Only Access**: Database encryption tied to USB serial number

## 📋 Requirements

### System Requirements
- **Operating System**: Windows 10+, macOS 10.14+, or Linux (Ubuntu 18.04+)
- **USB Port**: Available USB port for SanDisk device
- **Permissions**: Administrative access for initial setup (Windows/Linux)

### Hardware Requirements
- **SanDisk USB Device**: Any SanDisk USB storage device (Vendor ID: 0x0781)
- **USB Cable**: Standard USB connection

## 🛠️ Installation

### Windows Setup
1. Download the latest release from [GitHub Releases](https://github.com/TheEleventhAvatar/Anchor/releases)
2. Install the application
3. Run Zadig as Administrator:
   - Download [Zadig](https://github.com/pbatard/libwdi/releases) (zadig-2.9.exe)
   - Click Options → List All Devices
   - Select your SanDisk USB device
   - Replace driver with WinUSB
4. Launch Anchor

### macOS Setup
1. Download the latest macOS release
2. Install the application
3. Grant necessary permissions in System Preferences → Security & Privacy
4. Launch Anchor (no additional setup required)

### Linux Setup
1. Download the latest Linux release
2. Extract and install the application
3. Run the setup script:
   ```bash
   ./scripts/setup_linux.sh
   ```
4. Or manually create udev rule:
   ```bash
   echo 'SUBSYSTEM=="usb", ATTR{idVendor}=="0x0781", MODE="0666"' | sudo tee /etc/udev/rules.d/99-anchor-sandisk.rules
   sudo udevadm control --reload-rules
   sudo udevadm trigger
   ```
5. Launch Anchor

## 🎯 Usage

### Getting Started
1. **Insert SanDisk USB**: The application will automatically detect the device
2. **Initialize Database**: Click "Initialize Database" to create your encrypted storage
3. **Add Secure Data**: Enter and store your sensitive information
4. **Remove USB**: The application locks immediately when USB is disconnected

### Interface Overview

#### Locked State
- **Status**: Shows "Anchor - Locked" when no USB is detected
- **Setup Instructions**: Platform-specific setup guidance
- **Security Message**: Clear indication that database is inaccessible

#### Unlocked State
- **Dashboard**: Main interface when USB is connected
- **Database Management**: Initialize and manage encrypted storage
- **Data Entry**: Add and view secure information
- **Session Info**: Real-time USB connection status

### Data Management
- **Add Data**: Type information and press Enter or click "Add"
- **View Data**: Scroll through stored entries in reverse chronological order
- **Auto-Save**: Data is automatically encrypted and saved

## 🔧 Technical Architecture

### Backend (Rust/Tauri)
- **USB Detection**: nusb crate for cross-platform device enumeration
- **Database**: rusqlite with device-specific encryption
- **Concurrency**: tokio async runtime with 1-second polling interval
- **State Management**: Thread-safe Arc<Mutex<AppState>> pattern

### Frontend (React)
- **Event System**: Tauri event listeners for hardware status
- **State Management**: React hooks for UI state
- **Responsive Design**: Tailwind CSS for cross-platform consistency
- **TypeScript**: Full type safety throughout the application

### Platform-Specific Implementation
```rust
// Conditional compilation for platform-specific code
#[cfg(target_os = "windows")]
fn check_windows_permissions() { /* Windows-specific logic */ }

#[cfg(target_os = "macos")]
fn check_macos_permissions() { /* macOS-specific logic */ }

#[cfg(target_os = "linux")]
fn generate_udev_rule() { /* Linux-specific logic */ }
```

## 🔒 Security Model

### Threat Mitigation
- **Physical Security**: Requires physical USB device for access
- **Session Security**: Immediate lockout on USB removal
- **Data Protection**: Encrypted database with device-specific keys
- **No Credentials**: No passwords or tokens stored on disk

### Security Flow
1. **USB Detection**: Application polls for SanDisk devices every second
2. **Authentication**: USB presence serves as authentication factor
3. **Database Access**: SQLite connection only established when USB verified
4. **Session Termination**: Database connection closed immediately on USB removal

## 🐛 Troubleshooting

### Windows Issues
- **Driver Problems**: Ensure WinUSB driver installed via Zadig
- **Device Recognition**: Check Device Manager for proper driver
- **Permission Issues**: Run application as Administrator for first-time setup

### Linux Issues
- **USB Permissions**: Verify udev rule installation
- **Device Access**: Check `/dev/bus/usb/*/*` permissions
- **Detection**: Run `lsusb | grep 0781` to verify device detection

### macOS Issues
- **Permissions**: Check System Preferences → Security & Privacy
- **Device Recognition**: Verify USB device appears in System Information
- **App Permissions**: Allow USB device access in security settings

### General Issues
- **USB Not Detected**: Try different USB port or cable
- **Database Errors**: Re-initialize database after USB reconnection
- **App Crashes**: Check console logs for error messages

## 📝 Development

### Building from Source
```bash
# Clone the repository
git clone https://github.com/TheEleventhAvatar/Anchor.git
cd Anchor

# Install dependencies
npm install
cd src-tauri && cargo fetch && cd ..

# Development mode
npm run tauri dev

# Build for current platform
npm run tauri build
```

### Project Structure
```
Anchor/
├── src/                    # React frontend
│   ├── components/          # React components
│   ├── hooks/              # Custom React hooks
│   ├── utils/              # Utility functions
│   └── App.tsx            # Main application component
├── src-tauri/             # Rust backend
│   ├── src/                # Rust source code
│   ├── capabilities/        # Tauri capabilities
│   └── Cargo.toml         # Rust dependencies
├── scripts/               # Setup scripts
│   └── setup_linux.sh     # Linux udev setup
└── public/               # Static assets
```

### Dependencies
- **Rust**: nusb, rusqlite, tokio, tauri, serde
- **Node.js**: React, TypeScript, Tailwind CSS
- **Build Tools**: Tauri CLI, Vite

## 📄 License

This project is proprietary software. See the [LICENSE](LICENSE) file for details.

## 🤝 Contributing

Contributions are not accepted for this proprietary project. Please report issues through GitHub Issues for security considerations.

## 📞 Support

For support and security concerns:
- **Issues**: [GitHub Issues](https://github.com/TheEleventhAvatar/Anchor/issues)
- **Documentation**: [CROSS_PLATFORM_SETUP.md](CROSS_PLATFORM_SETUP.md)
- **Releases**: [GitHub Releases](https://github.com/TheEleventhAvatar/Anchor/releases)

---

**⚠️ Security Notice**: This application provides hardware-based security but should be used as part of a comprehensive security strategy. Always follow your organization's security policies and best practices.
