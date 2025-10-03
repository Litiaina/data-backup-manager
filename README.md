# Litiaina DBM

A lightweight tool for automating reliable and efficient file and data backups.  
It is designed for minimal overhead while providing durability, consistency, and flexibility.


## Features

- Full file/directory backup automation 
- Incremental and differential backups support
- Efficient storage and data deduplication


## Getting Started

### Installation

Clone the repository and build the project:

```bash

git clone https://github.com/Litiaina/data-backup-manager.git
cd data-backup-manager

```
## Usage

Run the binary directly after building. Note that this is a background task—no tray icons or windows will be visible when debug mode is disabled. Automated data backup will still run. During execution it will create log folder located on where the executable resides.

## Configuration

During execution debug mode is set to true as default to correctly debug and configure the source path, destination path and intervals.

Example config.ini:

```bash

[app]
# Set to true to prevent the console window from being hidden.
# To see the output, you must run this executable from an existing command prompt (cmd.exe or PowerShell).
# The '#![windows_subsystem = "windows"]' directive in the code means a new console window
# will NOT be created if you just double-click the .exe.
debug = true

[backup]
# Path to the directory you want to back up.
# IMPORTANT: You must change this path.
# Example: src_dir = "C:\\Users\\YourUser\\Documents"
src_dir = "C:\\replace\\with\\your\\source\\folder"

# Path to the directory where backups will be stored.
# IMPORTANT: You must change this path.
# Example: dst_dir = "D:\\Backups"
dst_dir = "D:\\replace\\with\\your\\backup\\destination"

# Backup interval. The app will back up the folder every X hours, Y minutes, Z seconds.
# Default is every 24 hours.
hours = 24
minutes = 0
seconds = 0

```

Contribution

Contributions are welcome! Feel free to open issues or pull requests.
Please document new features and include tests where possible.
