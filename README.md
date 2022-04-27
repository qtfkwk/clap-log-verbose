Here's a quick example integrating the popular logging crates [env\_logger] and [log] with the new
v3 [clap] crate to enable controlling the logging level via zero or more `-v` options and still
override with the `RUST_LOG` environment variable.

[clap]: https://crates.io/crates/clap
[env\_logger]: https://crates.io/crates/env_logger
[log]: https://crates.io/crates/log

```bash
git clone https://github.com/qtfkwk/clap-log-verbose.git
cd clap-log-verbose
cargo build
```

```text
$ ./target/debug/hello
[2022-04-27T00:00:00Z ERROR clap_log_verbose] test
$ ./target/debug/hello -v
[2022-04-27T00:00:00Z WARN  clap_log_verbose] test
$ ./target/debug/hello -vv
[2022-04-27T00:00:00Z INFO  clap_log_verbose] test
$ ./target/debug/hello -vvv
[2022-04-27T00:00:00Z DEBUG clap_log_verbose] test
$ ./target/debug/hello -vvvv
[2022-04-27T00:00:00Z TRACE clap_log_verbose] test
```

```text
$ RUST_LOG="error" ./target/debug/hello
[2022-04-27T00:00:00Z ERROR clap_log_verbose] test
$ RUST_LOG="warn" ./target/debug/hello
[2022-04-27T00:00:00Z WARN  clap_log_verbose] test
$ RUST_LOG="info" ./target/debug/hello
[2022-04-27T00:00:00Z INFO  clap_log_verbose] test
$ RUST_LOG="debug" ./target/debug/hello
[2022-04-27T00:00:00Z DEBUG clap_log_verbose] test
$ RUST_LOG="trace" ./target/debug/hello
[2022-04-27T00:00:00Z TRACE clap_log_verbose] test
```

* Note that the `RUST_LOG` environment variable **overrides** the `-v` option(s).

