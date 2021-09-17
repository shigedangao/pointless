# build process
# build the dependency independant of the code
FROM rust:1 as builder
ENV APP_PATH=pointless
RUN USER=root cargo new --bin pointless
WORKDIR ${APP_PATH}
COPY ./Cargo.toml ./Cargo.lock ./
RUN cargo build --release
RUN rm src/*.rs

# Use the code to compile the project w/o having to redownload the dependency
ADD . ./
RUN rm ./target/release/deps/pointless*
RUN cargo build --release

# actual container which run the app
FROM debian:buster
ARG APP=/usr/src/app

RUN apt-get purge ca-certificates 
RUN apt-get update && apt-get install -y openssl \
    ca-certificates

RUN useradd pointless

COPY --from=builder /pointless/target/release/pointless ${APP}/pointless

WORKDIR ${APP}

USER pointless

CMD ["./pointless"]