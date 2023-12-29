# Build from project root dir with: docker build . -f docker/rust/Dockerfile -t hanb
# Run with: docker run hanb --help; docker run -it hanb

FROM rust

COPY rust /hanb/
WORKDIR /hanb
RUN cargo build --release && \
    cp target/release/hanb /hanb && \
    cargo clean --release
ENTRYPOINT ["/hanb/hanb"]
