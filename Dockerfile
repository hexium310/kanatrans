# syntax = docker/dockerfile:1
FROM rust:1.83-slim-bookworm AS runtime
WORKDIR /usr/src
RUN --mount=type=cache,id=api:/var/cache/apt,target=/var/cache/apt \
    --mount=type=cache,id=api:/var/lib/apt/lists,target=/var/lib/apt/lists \
    apt-get update && apt-get install --no-install-recommends -y \
    clang \
    make \
    && rm -rf /var/lib/apt/lists/*

FROM runtime AS development

FROM runtime AS builder
RUN --mount=type=bind,source=crates,target=crates \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,id=api:/usr/local/cargo/registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=api:/usr/src/target,target=/usr/src/target \
    cargo build --release --no-default-features --features=vendored,server \
    && cp target/release/kanatrans /usr/local/bin/kanatrans

FROM scratch AS kanatrans
LABEL io.github.hexium310.kanatrans.app=kanatrans
LABEL org.opencontainers.image.source=https://github.com/hexium310/kanatrans
COPY --from=runtime /lib/x86_64-linux-gnu/libc.so* /lib/x86_64-linux-gnu/
COPY --from=runtime /lib/x86_64-linux-gnu/libgcc_s.so* /lib/x86_64-linux-gnu/
COPY --from=runtime /lib/x86_64-linux-gnu/libm.so* /lib/x86_64-linux-gnu/
COPY --from=runtime /lib64/ld-linux-x86-64.so* /lib64/
COPY --from=builder /usr/local/bin/kanatrans /usr/local/bin/kanatrans
ENTRYPOINT ["kanatrans", "--serve"]
