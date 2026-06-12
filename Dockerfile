# Dockerfile for Hermetic, Reproducible Static Compilation of ITS-OTM_public_attestation
FROM rust:1.78.0-slim AS builder

WORKDIR /usr/src/its-otm
COPY . .

RUN rustup target add x86_64-unknown-linux-musl && \
    apt-get update && apt-get install -y musl-tools && \
    cargo build --release --bin its_otm --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /usr/src/its-otm/target/x86_64-unknown-linux-musl/release/its_otm /its_otm
ENTRYPOINT ["/its_otm"]
