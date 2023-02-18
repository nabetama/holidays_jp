# holiday_jp
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fnabetama%2Fholidays_jp.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2Fnabetama%2Fholidays_jp?ref=badge_shield)


holiday_jp determines Japanese holiday.
The definition of holidays is based on this [csv file](https://www8.cao.go.jp/chosei/shukujitsu/syukujitsu.csv) provided by the Cabinet Office..

The holiday data is updated once a week by github action, but if you prefer to update it manually, run the following command

```sh
$ cargo run -- -g=true
$ cargo fmt # dont't have to do it
```

## Usage

### When used in a terminal like the shell command

```sh
# default
$ ./holiday_jp -d 20220101
20220101 is holiday(元日)

# the date format to pass as a arg
$ ./holiday_jp -d 2022/01/01 -f %Y/%m/%d
2022/01/01 is holiday(元日)

# help
$ ./holiday_jp -h
holiday_jp is determines holiday in Japan

Usage: holiday_jp [OPTIONS]

Options:
-d, --date <DATE> a date string, such as 20230211 (%Y%m%d) [default: ]
-g, --gen <BOOL> generate new syukujitsu data [possible values: true, false]
-f, --dateformat <DATE_FORMAT> Specify the date format to pass as a command line argument [default: %Y%m%d]
-h, --help Print help
-V, --version Print version
```


## License
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fnabetama%2Fholidays_jp.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2Fnabetama%2Fholidays_jp?ref=badge_large)