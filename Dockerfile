##Docker file for rust
FROM rust:latest
WORKDIR /app
COPY . .
RUN apt update
RUN apt install -y cmake --no-install-recommends
RUN cargo build --release
CMD ["./target/release/BastiBotMusicV4RS"]
