# Write a Dockerfile that builds an image to use Rust to build a manual memory management system
# that is as efficient as C. The program should be able to run on any Linux system.
# The Rust program should be called "rusty" and should be located in /usr/src/rusty.
# The executable file should be named "rusty" as well.
# Build the container with the tag "rusty".
# The container will be tested by running: docker run --rm -it rusty

FROM rust:1.42.0

WORKDIR /usr/src/rusty

COPY . .

RUN cargo build --release

CMD ["./target/release/rusty"]

