# ビルド
FROM ghcr.io/rust-cross/cargo-zigbuild AS builder

WORKDIR /app
COPY . .

RUN --mount=type=cache,target=${WORKDIR}/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/

RUN cargo zigbuild --release --target x86_64-unknown-linux-gnu

# 実行
FROM gcr.io/distroless/static:nonroot

WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-gnu/release/discord_utils /app/

USER nonroot

CMD ["/app/discord_utils"]
