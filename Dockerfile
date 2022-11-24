FROM rust:1.65.0-slim-bullseye

RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown

WORKDIR /usr/src/augmented_pathways
COPY ./augmented_pathways /usr/src/augmented_pathways/.
RUN cd /usr/src/augmented_pathways
EXPOSE 8080
CMD ["trunk", "serve", "--address", "0.0.0.0"]
