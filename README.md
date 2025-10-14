# holidays_jp

[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/nabetama/holidays_jp/rust.yml?branch=main)](https://github.com/nabetama/holidays_jp/actions)
[![GitHub](https://img.shields.io/github/license/nabetama/holidays_jp)](https://github.com/nabetama/holidays_jp/blob/main/LICENSE)
[![GitHub commit activity](https://img.shields.io/github/commit-activity/m/nabetama/holidays_jp)](https://github.com/nabetama/holidays_jp/pulse)
[![GitHub last commit](https://img.shields.io/github/last-commit/nabetama/holidays_jp)](https://github.com/nabetama/holidays_jp/commits/main)
[![Codacy Badge](https://app.codacy.com/project/badge/Grade/4c244ed513f94b74b6dfa7302c710165)](https://www.codacy.com/gh/nabetama/holidays_jp/dashboard?utm_source=github.com&utm_medium=referral&utm_content=nabetama/holidays_jp&utm_campaign=Badge_Grade)
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fnabetama%2Fholidays_jp.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2Fnabetama%2Fholidays_jp?ref=badge_shield)

**holidays_jp** is a command-line tool for determining Japanese national holidays. It provides a simple and efficient way to check if specific dates are holidays, list holidays within date ranges, and manage holiday data.

## Features

- 🗓️ **Flexible Date Formats**: Supports multiple date formats (YYYYMMDD, YYYY-MM-DD, YYYY/MM/DD, YYYY年MM月DD日, etc.)
- 📊 **Multiple Output Formats**: Human-readable, JSON, and quiet modes
- 📅 **Date Range Support**: List all holidays within a specified period
- 🔄 **Auto-Update**: Automatically updates holiday data from official sources
- 🌐 **Offline Support**: Works without internet connection after initial setup
- ⚡ **Fast & Lightweight**: Quick response times and minimal resource usage

## Data Source

The holiday data is based on the official [CSV file](https://www8.cao.go.jp/chosei/shukujitsu/syukujitsu.csv) provided by the Cabinet Office of Japan. The data is automatically updated once a week via [GitHub Actions](https://github.com/nabetama/holidays_jp/actions/workflows/scheduler.yml), ensuring you always have the latest holiday information.

## Installation

### From Source

```bash
git clone https://github.com/nabetama/holidays_jp.git
cd holidays_jp
cargo build --release
```

### Using Cargo

```bash
cargo install holidays_jp
```

## Quick Start

If your PC is connected to the Internet, you can obtain the latest Japanese national holiday data by executing the following command.

```sh
$ cargo run -- update
$ cargo fmt # dont't have to do it
```

## Usage

### Basic Commands

```sh
# Check today's date (default behavior)
$ ./holidays_jp
20251014 is not a holiday

# Check a specific date
$ ./holidays_jp check -d 20220101
20220101 is holiday(元日)

# Check with different date format
$ ./holidays_jp check -d 2022/01/01
2022/01/01 is holiday(元日)

# JSON output for scripting
$ ./holidays_jp check -d 2022-01-01 -o json
{"date":"2022-01-01","is_holiday":true,"holiday_name":"元日"}

# Quiet output (holiday name only)
$ ./holidays_jp check -d 2022-01-01 -o quiet
元日
```

### List Holidays in a Range

```sh
# List holidays in January 2023
$ ./holidays_jp list --start 2023-01-01 --end 2023-01-31
Holidays in range (2023-01-01 to 2023-01-31):
  2023-01-01 - 元日
  2023-01-02 - 休日
  2023-01-09 - 成人の日

# JSON output for programmatic use
$ ./holidays_jp list --start 2023-01-01 --end 2023-01-31 -o json
{
  "start_date": "2023-01-01",
  "end_date": "2023-01-31",
  "holidays": [
    {
      "date": "2023-01-01",
      "is_holiday": true,
      "holiday_name": "元日"
    },
    {
      "date": "2023-01-02",
      "is_holiday": true,
      "holiday_name": "休日"
    },
    {
      "date": "2023-01-09",
      "is_holiday": true,
      "holiday_name": "成人の日"
    }
  ]
}

# List all holidays in 2023
$ ./holidays_jp list --start 2023/01/01 --end 2023/12/31
```

### Update Holiday Data

```sh
# Update to the latest holiday data
$ ./holidays_jp update
🔄 Updating holiday data from official source...
✅ Holiday data updated successfully!
```

### Get Help

```sh
# General help
$ ./holidays_jp --help

# Command-specific help
$ ./holidays_jp check --help
$ ./holidays_jp list --help
$ ./holidays_jp update --help
```

## Supported Date Formats

The tool automatically detects and supports various date formats:

- `YYYYMMDD` (e.g., `20230101`)
- `YYYY-MM-DD` (e.g., `2023-01-01`)
- `YYYY/MM/DD` (e.g., `2023/01/01`)
- `YYYY年MM月DD日` (e.g., `2023年1月1日`)
- `MM/DD/YYYY` (e.g., `01/01/2023`)
- `DD/MM/YYYY` (e.g., `01/01/2023`)
- `YYYY.MM.DD` (e.g., `2023.01.01`)

## Output Formats

- **human** (default): Human-readable format with clear messages
- **json**: Structured JSON output for programmatic use
- **quiet**: Minimal output showing only holiday names

## License

[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fnabetama%2Fholidays_jp.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2Fnabetama%2Fholidays_jp?ref=badge_large)
