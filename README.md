# pmtud-rs

[![Build Status](https://travis-ci.org/thekuwayama/pmtud-rs.svg?branch=master)](https://travis-ci.org/thekuwayama/pmtud-rs)
[![MIT licensed](https://img.shields.io/badge/license-MIT-brightgreen.svg)](https://raw.githubusercontent.com/thekuwayama/pmtud-rs/master/LICENSE.txt)

`pmtud-rs` is CLI for Path MTU Discovery.

`pmtud-rs` sends ICMP Echo Request packets with DF bit.

## Usage

You can build and run `pmtud-rs` with the following:

```bash
$ git clone git@github.com:thekuwayama/pmtud-rs.git

$ cd pmtud-rs

$ cargo build

$ sudo ./target/debug/pmtud-rs 1.1.1.1
```

## License

The CLI is available as open source under the terms of the [MIT License](http://opensource.org/licenses/MIT).
