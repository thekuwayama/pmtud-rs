# pmtud-rs

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
