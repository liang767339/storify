# Storify

A unified command-line tool for managing object storage with HDFS-like interface.

## Features

- **Multi-cloud support**: OSS, S3, MinIO, COS, and local filesystem
- **HDFS-compatible commands**: Familiar interface for Hadoop users
- **Unified configuration**: Single tool for all storage providers
- **High performance**: Async I/O with progress reporting
- **Cross-platform**: Works on Linux, macOS, and Windows

## Installation

### From Source

```bash
git clone https://github.com/QuakeWang/storify.git
cd storify
cargo build --release
```

The binary will be available at `target/release/storify`.

### From Cargo (when published)

```bash
cargo install storify
```

## Configuration

Set your storage provider and credentials using environment variables:

```bash
# Choose provider: oss, s3, minio, cos, or fs
export STORAGE_PROVIDER=oss

# Common configuration
export STORAGE_BUCKET=your-bucket
export STORAGE_ACCESS_KEY_ID=your-access-key
export STORAGE_ACCESS_KEY_SECRET=your-secret-key

# Optional
export STORAGE_ENDPOINT=your-endpoint
export STORAGE_REGION=your-region
```

### Provider-specific variables (legacy support)

```bash
# OSS
OSS_BUCKET, OSS_ACCESS_KEY_ID, OSS_ACCESS_KEY_SECRET

# AWS S3  
AWS_S3_BUCKET, AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY

# MinIO
MINIO_BUCKET, MINIO_ACCESS_KEY, MINIO_SECRET_KEY

# COS
COS_BUCKET, COS_SECRET_ID, COS_SECRET_KEY

# Filesystem
STORAGE_ROOT_PATH=./storage
```

## Usage

```bash
# List directory contents
storify ls path/to/dir
storify ls path/to/dir -L          # detailed format
storify ls path/to/dir -R          # recursive

# Download files/directories  
storify get remote/path local/path

# Upload files/directories
storify put local/path remote/path
storify put local/dir remote/dir -R # recursive

# Copy within storage
storify cp source/path dest/path

# Mv within storage
storify mv source/path dest/path
storify mv path/src_file  path/dest_file

# Display file contents
storify cat path/to/file

# Show disk usage
storify du path/to/dir
storify du path/to/dir -s          # summary only

# Delete files/directories
storify rm path/to/file
storify rm path/to/dir -R          # recursive

# Show object metadata
storify stat path/to/file           # human-readable
storify stat path/to/file --raw     # raw key=value lines (compat with opendal-mkdir)
storify stat path/to/file --json    # JSON output
```

## Command Reference

| Command | Description | Options |
|---------|-------------|---------|
| `ls` | List directory contents | `-L` (detailed), `-R` (recursive) |
| `get` | Download files from remote | |
| `put` | Upload files to remote | `-R` (recursive) |
| `cp` | Copy files within storage | |
| `mv` | Rename files, or move files | |
| `mkdir` | Create directories | `-p` (parents) |
| `cat` | Display file contents | |
| `rm` | Delete files/directories | `-R` (recursive), `-f` (force) |
| `du` | Show disk usage | `-s` (summary only) |
| `stat` | Show object metadata | `--json`, `--raw` |

## Architecture

Built on [OpenDAL](https://github.com/apache/opendal) for unified storage access.

```
┌───────────────────────┐
│      Storify CLI      │
├───────────────────────┤
│    Storage Client     │
├───────────────────────┤
│       OpenDAL         │
├───────────────────────┤
│ OSS │ S3 │ COS │ MinIO│
└───────────────────────┘
```

## Development

### Prerequisites

- Rust 1.80+
- Cargo
- Git

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

## License

Apache License 2.0 - see [LICENSE](LICENSE) for details.
# Storify

A unified command-line tool for managing object storage with HDFS-like interface.

## Features

- **Multi-cloud support**: OSS, S3, MinIO, COS, and local filesystem
- **HDFS-compatible commands**: Familiar interface for Hadoop users
- **Unified configuration**: Single tool for all storage providers
- **High performance**: Async I/O with progress reporting
- **Cross-platform**: Works on Linux, macOS, and Windows

## Installation

### From Source

```bash
git clone https://github.com/QuakeWang/storify.git
cd storify
cargo build --release
```

The binary will be available at `target/release/storify`.

### From Cargo (when published)

```bash
cargo install storify
```

## Configuration

Set your storage provider and credentials using environment variables:

```bash
# Choose provider: oss, s3, minio, cos, or fs
export STORAGE_PROVIDER=oss

# Common configuration
export STORAGE_BUCKET=your-bucket
export STORAGE_ACCESS_KEY_ID=your-access-key
export STORAGE_ACCESS_KEY_SECRET=your-secret-key

# Optional
export STORAGE_ENDPOINT=your-endpoint
export STORAGE_REGION=your-region
```

### Provider-specific variables (legacy support)

```bash
# OSS
OSS_BUCKET, OSS_ACCESS_KEY_ID, OSS_ACCESS_KEY_SECRET

# AWS S3  
AWS_S3_BUCKET, AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY

# MinIO
MINIO_BUCKET, MINIO_ACCESS_KEY, MINIO_SECRET_KEY

# COS
COS_BUCKET, COS_SECRET_ID, COS_SECRET_KEY

# Filesystem
STORAGE_ROOT_PATH=./storage
```

## Usage

```bash
# List directory contents
storify ls path/to/dir
storify ls path/to/dir -L          # detailed format
storify ls path/to/dir -R          # recursive

# Download files/directories  
storify get remote/path local/path

# Upload files/directories
storify put local/path remote/path
storify put local/dir remote/dir -R # recursive

# Copy within storage
storify cp source/path dest/path

# Mv within storage
storify mv source/path dest/path
storify mv path/src_file  path/dest_file

# Display file contents
storify cat path/to/file

# Show disk usage
storify du path/to/dir
storify du path/to/dir -s          # summary only

# Delete files/directories
storify rm path/to/file
storify rm path/to/dir -R          # recursive

# Show object metadata
storify stat path/to/file           # human-readable
storify stat path/to/file --raw     # raw key=value lines (compat with opendal-mkdir)
storify stat path/to/file --json    # JSON output
```

## Command Reference

| Command | Description | Options |
|---------|-------------|---------|
| `ls` | List directory contents | `-L` (detailed), `-R` (recursive) |
| `get` | Download files from remote | |
| `put` | Upload files to remote | `-R` (recursive) |
| `cp` | Copy files within storage | |
| `mv` | Rename files, or move files | |
| `mkdir` | Create directories | `-p` (parents) |
| `cat` | Display file contents | |
| `rm` | Delete files/directories | `-R` (recursive), `-f` (force) |
| `du` | Show disk usage | `-s` (summary only) |
| `stat` | Show object metadata | `--json`, `--raw` |

## Architecture

Built on [OpenDAL](https://github.com/apache/opendal) for unified storage access.

```
┌───────────────────────┐
│      Storify CLI      │
├───────────────────────┤
│    Storage Client     │
├───────────────────────┤
│       OpenDAL         │
├───────────────────────┤
│ OSS │ S3 │ COS │ MinIO│
└───────────────────────┘
```

## Development

### Prerequisites

- Rust 1.80+
- Cargo
- Git

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

## License

Apache License 2.0 - see [LICENSE](LICENSE) for details.
