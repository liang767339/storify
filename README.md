# Ossify

## Overview

Ossify is a unified tool for managing object storage with HDFS-like interface. It supports multiple cloud storage providers including Alibaba Cloud OSS, AWS S3, and local filesystem, providing a consistent command-line experience across different platforms.

## Features

- **Multi-cloud support**: OSS, S3, and local filesystem
- **HDFS-compatible interface**: Familiar commands for Hadoop users
- **Unified configuration**: Single tool for all your object storage needs
- **Recursive operations**: Directory traversal and batch operations
- **Preserve directory structure**: Maintains hierarchy during transfers
- **Parallel async I/O**: High-performance operations
- **Cross-platform support**: Linux/macOS/Windows

## Supported Storage Providers

| Provider | Status | Environment Variables |
|----------|--------|----------------------|
| Alibaba Cloud OSS | ✅ | `OSS_*` or `STORAGE_*` |
| AWS S3 | ✅ | `AWS_*` or `STORAGE_*` |
| Local Filesystem | ✅ | `STORAGE_ROOT_PATH` |

## Installation

### From Source

```bash
git clone https://github.com/QuakeWang/ossify.git
cd ossify
cargo build --release

# Install to system path (optional)
sudo cp target/release/ossify /usr/local/bin/
```

## Configuration

### Universal Environment Variables (Recommended)

```bash
# Storage provider (default: oss)
export STORAGE_PROVIDER=oss|s3|fs

# Common configuration
export STORAGE_BUCKET=your_bucket_name
export STORAGE_ACCESS_KEY_ID=your_access_key_id
export STORAGE_ACCESS_KEY_SECRET=your_access_key_secret

# Optional settings
export STORAGE_REGION=your_region
export STORAGE_ENDPOINT=your_custom_endpoint
export STORAGE_ROOT_PATH=./storage           # For filesystem
```

### Provider-Specific Variables (Legacy Support)

#### Alibaba Cloud OSS
```bash
export OSS_BUCKET=your_bucket
export OSS_ACCESS_KEY_ID=your_access_key_id
export OSS_ACCESS_KEY_SECRET=your_access_key_secret
export OSS_ENDPOINT=oss-cn-hangzhou.aliyuncs.com  # Optional
export OSS_REGION=cn-hangzhou                     # Optional
```

#### AWS S3
```bash
export AWS_S3_BUCKET=your_bucket
export AWS_ACCESS_KEY_ID=your_access_key_id
export AWS_SECRET_ACCESS_KEY=your_secret_key
export AWS_DEFAULT_REGION=us-west-2            # Optional
```

## Usage

Ossify provides HDFS-compatible commands for object storage operations:

### List Directory Contents

```bash
# Basic listing
ossify -l "path/to/directory"

# Detailed listing (with size, date, type)
ossify -l "path/to/directory" -L

# Recursive listing
ossify -l "path/to/directory" -R

# Recursive detailed listing
ossify -l "path/to/directory" -L -R
```

**Example output:**
```
# Basic format
path/to/directory/file1.txt
path/to/directory/subdir/file2.log

# Detailed format (-L)
FILE         1.2K 2024-01-15T10:30:45+00:00 path/to/directory/file1.txt
DIR             - Unknown                   path/to/directory/subdir/
```

### Download Files

```bash
# Download files/directories
ossify -g "remote/path" "local/destination"

# Examples
ossify -g "data/2024/reports/" "./local/reports/"
ossify -g "backup.zip" "./local/backup.zip"
```

**Example output:**
```
Downloaded: data/2024/reports/jan.csv → ./local/reports/jan.csv
Downloaded: data/2024/reports/feb.csv → ./local/reports/feb.csv
```

### Disk Usage Statistics

```bash
# Show disk usage for each item
ossify -d "path/to/directory"

# Show summary only
ossify -d "path/to/directory" -s
```

**Example output:**
```
# Detailed format
1.2M path/to/directory/large_file.zip
500K path/to/directory/data.csv
2.1M path/to/directory/images/

# Summary format (-s)
3.8M path/to/directory/
Total files: 42
```

## Multi-Cloud Examples

### Switch Between Providers

```bash
# Use Alibaba Cloud OSS
export STORAGE_PROVIDER=oss
ossify -l "my-data/"

# Use AWS S3
export STORAGE_PROVIDER=s3
ossify -l "my-data/"

# Use local filesystem for testing
export STORAGE_PROVIDER=fs
export STORAGE_ROOT_PATH=./local-storage
ossify -l "./"
```

### Cross-Cloud Data Transfer

```bash
# Download from OSS
export STORAGE_PROVIDER=oss
ossify -g "source/data/" "./temp/"

# Upload to S3 (when upload feature is available)
export STORAGE_PROVIDER=s3
# ossify -p "./temp/" "destination/data/"
```

## Command Reference

| Command | HDFS Equivalent | Description |
|---------|----------------|-------------|
| `ossify -l <path>` | `hdfs dfs -ls <path>` | List directory contents |
| `ossify -l <path> -L` | `hdfs dfs -ls -l <path>` | List with details |
| `ossify -l <path> -R` | `hdfs dfs -ls -R <path>` | Recursive list |
| `ossify -g <src> <dst>` | `hdfs dfs -get <src> <dst>` | Download files |
| `ossify -d <path>` | `hdfs dfs -du <path>` | Show disk usage |
| `ossify -d <path> -s` | `hdfs dfs -du -s <path>` | Show usage summary |

## Options

| Option | Description |
|--------|-------------|
| `-l, --ls <PATH>` | List directory contents |
| `-g, --get <REMOTE> <LOCAL>` | Download files from remote to local |
| `-d, --du <PATH>` | Show disk usage statistics |
| `-L, --long` | Show detailed information (long format) |
| `-R, --recursive` | Process directories recursively |
| `-s, --summary` | Show summary only (for du command) |
| `-h, --help` | Print help information |
| `-V, --version` | Print version information |

## Architecture

Ossify leverages [OpenDAL](https://github.com/apache/incubator-opendal) to provide unified access to different storage services:

```
┌─────────────────┐
│   Ossify CLI    │
├─────────────────┤
│ Storage Client  │
├─────────────────┤
│    OpenDAL      │
├─────────────────┤
│ OSS │ S3 │ FS   │
└─────────────────┘
```

## Development

### Adding New Storage Providers

1. Add the provider to `StorageProvider` enum
2. Implement configuration loading in `main.rs`
3. Add OpenDAL service configuration in `storage.rs`
4. Update documentation

### Running Tests

```bash
# Build the project
cargo build

# Run with local filesystem for testing
export STORAGE_PROVIDER=fs
export STORAGE_ROOT_PATH=./test-data
mkdir -p test-data
echo "test" > test-data/sample.txt

# Test commands
./target/debug/ossify -l "./"
./target/debug/ossify -l "./" -L
./target/debug/ossify -d "./" -s
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Roadmap

- [x] Multi-cloud storage support
- [x] HDFS-compatible interface
- [x] Unified configuration system
- [ ] File upload functionality (`-p/--put`)
- [ ] File deletion (`-r/--rm`)
- [ ] Directory creation (`--mkdir`)
- [ ] File movement/copy (`--mv`, `--cp`)
- [ ] Configuration file support
- [ ] Batch operations
- [ ] Progress indicators
- [ ] Compression support