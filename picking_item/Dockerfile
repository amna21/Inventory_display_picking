FROM rust:1.55 as cargo-build

WORKDIR /usr/src/app
COPY ./src src
COPY ./resources resources
COPY Cargo.lock .
COPY Cargo.toml .
RUN mkdir .cargo
RUN cargo vendor > .cargo/config
RUN cargo build --release
RUN cargo install --path . --verbose


FROM ubuntu:20.04
COPY --from=cargo-build /usr/local/cargo/bin/picking_item /bin
COPY --from=cargo-build /usr/src/app/ /src
COPY --from=cargo-build /usr/local/cargo/ /usr/local/cargo
