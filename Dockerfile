FROM rust:1.67.0
WORKDIR /usr/src/exporter_test
COPY . .
RUN cargo install --path .
CMD ["exporter_test"]