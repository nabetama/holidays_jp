# holiday-ja

holiday-ja determines Japanese holiday.
The definition of holidays is based on this [csv file](https://www8.cao.go.jp/chosei/shukujitsu/syukujitsu.csv) provided by the Cabinet Office..

The holiday data is updated once a week by github action, but if you prefer to update it manually, run the following command

```
$ cargo run -- -g=true
```

## HOW TO USE

### When used in a terminal like the shell command

```sh
$ holiday-ja -d 2022/01/01
2022/01/01 is holiday (元日)

$ holiday-ja -h
holiday-js is determines holiday in Japan

Usage: holiday-ja [OPTIONS]

Options:
  -d, --date <DATE>  a date string, such as 2023/02/11 (%Y/%m/%d) [default: ]
  -g, --gen <BOOL>   generate new syukujitsu data [possible values: true, false]
  -h, --help         Print help
  -V, --version      Print version
```

### When used as a library

```rs
fn test_is_holiday() {
    let dt = NaiveDate::parse_from_str("2023/01/01", "%Y/%m/%d");
    match dt {
        Ok(dt) => assert_eq!(is_holiday(dt), true),
        Err(err) => eprintln!("{:?}", err),
    }
}
```
