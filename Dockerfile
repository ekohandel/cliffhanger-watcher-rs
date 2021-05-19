FROM rust AS build

COPY ./ ./
RUN cargo build --release

FROM rust AS runtime
COPY --from=build ./target/release/cliffhanger_watcher /app/cliffhanger_watcher
WORKDIR /app
ENTRYPOINT ["./cliffhanger_watcher"]
