FROM rust:1.65.0-slim-bullseye as build

RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown

WORKDIR /usr/src/augmented_pahtways
COPY . /usr/src/augmented_pahtways
RUN cd /usr/src/augmented_pahtways/frontend && trunk build --release
RUN cd /usr/src/augmented_pahtways/backend && cargo build --release

RUN cp /usr/src/augmented_pahtways/target/release/backend /usr/local/bin/backend
RUN cp -r /usr/src/augmented_pahtways/frontend/dist /usr/local/bin/dist

EXPOSE 3000
WORKDIR /usr/local/bin
CMD ["backend"]
