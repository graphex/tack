FROM rust:1.65 as builder

RUN USER=root cargo new --bin tack
WORKDIR ./tack
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/tack*
RUN cargo build --release


FROM debian:bullseye-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /tack/target/release/tack ${APP}/tack
COPY ./scripts/healthcheck.sh ${APP}/healthcheck.sh
RUN chown -R $APP_USER:$APP_USER ${APP} && chmod u+x ${APP}/healthcheck.sh

USER $APP_USER
WORKDIR ${APP}
