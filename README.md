# holiday_jp

holiday_jp determines Japanese holiday.
The definition of holidays is based on this [csv file](https://www8.cao.go.jp/chosei/shukujitsu/syukujitsu.csv) provided by the Cabinet Office..

The holiday data is updated once a week by github action, but if you prefer to update it manually, run the following command

```
$ cargo run -- -g=true
```

## HOW TO USE

### When used in a terminal like the shell command

```sh
$ holiday_jp -d 2022/01/01
2022/01/01 is holiday (元日)

$ holiday_jp -h
holiday_jp is determines holiday in Japan

Usage: holiday_jp [OPTIONS]

Options:
  -d, --date <DATE>  a date string, such as 2023/02/11 (%Y/%m/%d) [default: ]
  -g, --gen <BOOL>   generate new syukujitsu data [possible values: true, false]
  -h, --help         Print help
  -V, --version      Print version
```
