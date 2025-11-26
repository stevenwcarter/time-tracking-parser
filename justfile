test:
    watchexec -e rs,toml cargo nextest run

cover:
    cargo llvm-cov --lcov --output-path lcov.info
