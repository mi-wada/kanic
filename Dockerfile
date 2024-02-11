FROM --platform=linux/amd64 rust:1.76.0

WORKDIR /app

COPY ./Cargo.toml ./Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs
RUN cargo build

COPY . .
