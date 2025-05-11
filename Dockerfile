# ビルド
FROM ghcr.io/rust-cross/cargo-zigbuild AS builder

WORKDIR /app
COPY . .

RUN --mount=type=cache,target=${WORKDIR}/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/

RUN cargo zigbuild --release --target x86_64-unknown-linux-gnu
RUN cargo install --path .

FROM ubuntu AS press

WORKDIR /works
RUN apt-get install -y upx

COPY --from=builder /usr/local/cargo/bin/discord_utils /works/
RUN upx /works/discord_utils

# 実行
FROM gcr.io/distroless/static:nonroot

WORKDIR /app
COPY --from=press /works/discord_utils /app/

USER nonroot

CMD ["/app/discord_utils"]
