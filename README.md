# holidays_jp

[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/nabetama/holidays_jp/rust.yml?branch=main)](https://github.com/nabetama/holidays_jp/actions)
[![GitHub](https://img.shields.io/github/license/nabetama/holidays_jp)](https://github.com/nabetama/holidays_jp/blob/main/LICENSE)
[![GitHub commit activity](https://img.shields.io/github/commit-activity/m/nabetama/holidays_jp)](https://github.com/nabetama/holidays_jp/pulse)
[![GitHub last commit](https://img.shields.io/github/last-commit/nabetama/holidays_jp)](https://github.com/nabetama/holidays_jp/commits/main)
[![Codacy Badge](https://app.codacy.com/project/badge/Grade/4c244ed513f94b74b6dfa7302c710165)](https://www.codacy.com/gh/nabetama/holidays_jp/dashboard?utm_source=github.com&utm_medium=referral&utm_content=nabetama/holidays_jp&utm_campaign=Badge_Grade)
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fnabetama%2Fholidays_jp.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2Fnabetama%2Fholidays_jp?ref=badge_shield)

holidays_jp determines Japanese holiday.
The definition of Japanese national holidays is based on this [csv file](https://www8.cao.go.jp/chosei/shukujitsu/syukujitsu.csv) provided by the Cabinet Office..

The holiday data is updated once a week by [github action](https://github.com/nabetama/holidays_jp/actions/workflows/scheduler.yml). Thereby, holiday data is included in the repository. Therefore, this tool can be used offline.
If your PC is connected to the Internet, you can obtain the latest Japanese national holiday data by executing the following command.

```sh
$ cargo run -- -g=true
$ cargo fmt # dont't have to do it
```

## Usage

### When used in a terminal like the shell command

```sh
# default
$ ./holidays_jp -d 20220101
20220101 is holiday(元日)

# the date format to pass as a arg
$ ./holidays_jp -d 2022/01/01 -f %Y/%m/%d
2022/01/01 is holiday(元日)

# help
$ ./target/release/holidays_jp -h
holidays_jp is determines holiday in Japan

Usage: holidays_jp [OPTIONS]

Options:
  -d, --date <DATE>               a date string, such as 20230211 (%Y%m%d) [default: ]
  -g, --gen <BOOL>                generates a new Japanese national holidays data [possible values: true, false]
  -f, --dateformat <DATE_FORMAT>  Specify the date format to pass as a command line argument [default: %Y%m%d]
  -h, --help                      Print help
  -V, --version                   Print version
```

## License

[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fnabetama%2Fholidays_jp.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2Fnabetama%2Fholidays_jp?ref=badge_large)
