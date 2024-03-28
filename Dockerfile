FROM clux/muslrust:stable as build-env
WORKDIR /app
COPY . /app
ENV SQLX_OFFLINE=true
RUN cargo build --release --package backend

FROM gcr.io/distroless/static-debian12
COPY --from=build-env /app/target/x86_64-unknown-linux-musl/release/backend /
CMD ["./backend"]
