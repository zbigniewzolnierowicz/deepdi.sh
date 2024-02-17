FROM rust:1.76 as build-env
WORKDIR /app
COPY . /app
ENV SQLX_OFFLINE=true
RUN cargo build --release --package backend

FROM gcr.io/distroless/cc-debian12
COPY --from=build-env /app/target/release/backend /
CMD ["./backend"]
