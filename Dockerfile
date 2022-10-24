FROM rust:1.63

RUN mkdir -p /usr/src/augmented_pathways
WORKDIR /usr/src/augmented_pathways
COPY ./augmented_pathways /usr/src/augmented_pathways
RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown
RUN cargo build -r 
EXPOSE 3000
CMD ["trunk", "serve"]
