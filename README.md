# Ossify

## Overview

Ossify is a tool for managing OSS (Object Storage Service) files. It simplifies operations such as listing, downloading, and counting files in an OSS bucket.

## Features

- Recursive directory traversal
- Preserve directory structure when downloading
- Parallel async I/O operations
- Cross-platform support (Linux/macOS/Windows) 

## Usage

### Set Environment Variables

Before using Ossify, set the following environment variables:

```bash
# Required
export OSS_BUCKET=your_bucket
export OSS_ACCESS_KEY_ID=your_access_key_id
export OSS_ACCESS_KEY_SECRET=your_access_key_secret

# Optional (default: https://oss-cn-hangzhou.aliyuncs.com)
export OSS_ENDPOINT=your_custom_endpoint
```

### Install & Build

```bash
git clone git@github.com:QuakeWang/ossify.git
cd ossify
cargo build --release

# Install to system path (optional)
sudo cp target/release/ossify /usr/local/bin/
```

### Commands

#### List files recursively

```bash
ossify list "oss/path/to/directory"

# Example output:
# File: oss/path/to/directory/file1.txt
# File: oss/path/to/directory/subdir/file2.log
```

#### Download files with directory structure

```bash
ossify download \
  --remote_path "oss/path/to/source" \
  --local_path "./local/destination"

# Example output:
# Downloaded: oss/path/to/source/data.csv → ./local/destination/data.csv
# Downloaded: oss/path/to/source/images/logo.png → ./local/destination/images/logo.png
```

#### Count files recursively

```bash
ossify count --path "oss/path/to/directory"

# Example output:
# File count: 42
```

## TODO

- [ ] Add more tests
- [ ] Add more examples
- [ ] Add more features
- [ ] Add more documentation
