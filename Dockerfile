FROM rust:1.67.1-alpine3.17 as builder
WORKDIR /usr/src/app
RUN USER=root cargo init --bin
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN apk add --no-cache tzdata openssl-dev musl-dev upx
RUN cargo build --release && rm src/*.rs
COPY ./src ./src
RUN rm /usr/src/app/target/release/deps/git_reminder_bot* \
    && cargo build --release \
    && upx --best --lzma target/release/git-reminder-bot

FROM scratch
WORKDIR /usr/local/bin
COPY --from=builder /usr/src/app/target/release/git-reminder-bot .
COPY --from=builder /usr/share/zoneinfo /usr/share/zoneinfo
ENV TZ=UTC
CMD ["./git-reminder-bot"]
