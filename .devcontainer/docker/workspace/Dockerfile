ARG RUST_VERSION
ARG APP_CODE_PATH_CONTAINER
FROM rust:${RUST_VERSION}

RUN adduser app
USER app

ARG APP_CODE_PATH_CONTAINER
WORKDIR ${APP_CODE_PATH_CONTAINER}

RUN cargo install diesel_cli --no-default-features --features postgres
