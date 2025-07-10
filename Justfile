format:
  taplo fmt
  cargo +nightly fmt --all
  leptosfmt .
lint:
  taplo fmt --check
  cargo +nightly fmt --all -- --check
  leptosfmt --check .
  cargo +nightly clippy --all -- -D warnings -A clippy::derive_partial_eq_without_eq -D clippy::unwrap_used -D clippy::uninlined_format_args
  cargo machete
test:
  cargo test