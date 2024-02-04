test:
    cargo test --lib

integration-test:
    cargo test --test integration_test

coverage:
    cargo tarpaulin -- --test-threads 1