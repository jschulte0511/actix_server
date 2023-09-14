####################################################################################################
## Builder
####################################################################################################
FROM rust:1.68 as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .
RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################
FROM debian:bullseye-slim

WORKDIR /usr/local/bin/app

RUN apt-get update && apt-get install && apt-get install -y procps
RUN apt install -y net-tools && apt install -y systemd
RUN rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release ./

CMD ["/usr/local/bin/app/actix_server"]
EXPOSE 8000