FROM rust:1.65.0-slim-bullseye as build

RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown

WORKDIR /usr/src/augmented_pathways
COPY . /usr/src/augmented_pathways
RUN cd /usr/src/augmented_pathways/frontend && trunk build --release
RUN cd /usr/src/augmented_pathways/backend && cargo build --release

RUN cp /usr/src/augmented_pathways/target/release/backend /usr/local/bin/backend
RUN cp -r /usr/src/augmented_pathways/frontend/dist /usr/local/bin/dist

EXPOSE 3000
WORKDIR /usr/local/bin
CMD ["backend"]
