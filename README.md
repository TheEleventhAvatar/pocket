# Pocket Desktop

A production-grade desktop companion app for Pocket - a tiny AI device that lives on your phone and turns everything you say and hear into clear notes, action items, and search. We combine custom hardware, AI, and software to build the best note-taking experience for people who talk and move fast.

Built with Tauri 2.x, Rust, and React + TypeScript.

## Features

- **Offline-first design**: Local SQLite database with phone app sync capabilities
- **Real-time simulation**: Mock transcript generation every 10 seconds when Pocket is connected
- **System tray integration**: Show/hide window, quit application
- **Native notifications**: Desktop alerts for new transcripts from Pocket
- **Pocket status monitoring**: Connection status, battery level, last seen
- **Sync management**: Manual sync, offline/online modes
- **Clean architecture**: Modular Rust backend with separate concerns

## Product Vision

Pocket is a tiny AI device that:
- Lives on your phone
- Captures everything you say and hear
- Transforms speech into clear notes and action items
- Provides powerful search capabilities
- Built for people who talk and move fast

The desktop companion provides:
- Larger screen for reviewing and organizing notes
- Advanced editing capabilities
- Backup and sync management
- Desktop notifications for new captures

## Architecture

### Backend (Rust)
- `db.rs`: SQLite database operations
- `commands.rs`: Tauri commands and business logic
- `main.rs`: Application setup and system tray

### Frontend (React + TypeScript)
- `types.ts`: TypeScript interfaces
- `hooks/useTauriCommands.ts`: Tauri API wrapper
- `components/`: React components
  - `Sidebar.tsx`: Phone status and sync controls
  - `TranscriptList.tsx`: Transcript listing
  - `TranscriptDetail.tsx`: Transcript viewer
  - `AddTranscriptForm.tsx`: Manual transcript creation

## Database Schema

```sql
CREATE TABLE transcripts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TEXT NOT NULL,
    synced INTEGER NOT NULL DEFAULT 0
);
```

## Getting Started

### Prerequisites
- Rust 1.70+
- Node.js 18+
- npm or yarn

### Installation

1. Clone the repository
2. Install dependencies:
   ```bash
   npm install
   ```

### Development

1. Start the development server:
   ```bash
   npm run tauri dev
   ```

2. The application will:
   - Open the main window
   - Start backend services
   - Begin generating mock transcripts when Pocket is connected

### Building

1. Build for production:
   ```bash
   npm run build
   ```

2. Create distributable:
   ```bash
   npm run tauri build
   ```

## Usage

### Pocket Simulation
1. Click "Connect Phone" in the sidebar to simulate Pocket device connection
2. Mock transcripts will be generated every 10 seconds
3. View Pocket status including battery level

### Sync Management
1. Toggle between online/offline modes
2. Click "Sync Now" to mark all transcripts as synced
3. View unsynced count in real-time

### Transcript Management
1. Click any transcript to view details
2. Add manual transcripts with the form
3. Mark individual transcripts as synced

## Technical Details

### Background Tasks
- Runs every 10 seconds when Pocket is connected
- Generates mock transcripts with realistic content
- Updates Pocket status timestamps

### State Management
- React hooks for local state
- Tauri commands for backend operations
- Real-time polling every 5 seconds

### Styling
- Pure CSS, no frameworks
- Modern, clean design
- Responsive layout with sidebar

## Deployment

### GitHub Releases (Recommended)

1. **Tag a new release:**
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

2. **Automatic builds** will create installers for:
   - Windows (.msi)
   - macOS (.dmg)
   - Linux (.deb)

3. **Download installers** from GitHub Releases

### Auto-Updates

The app includes Tauri Updater for automatic updates when new releases are published.

### Local Build

```bash
# Build for your platform
npm run tauri build

# Find installers in:
ls src-tauri/target/release/bundle/
```

## License

MIT License - see LICENSE file for details
