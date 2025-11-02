# syntax=docker/dockerfile:1

# ===============================================================================
# BUILDER
# ===============================================================================
FROM rust:slim-bullseye AS builder

# install deps
RUN apt-get update && apt-get install -y git libssl-dev pkg-config

# get sources
RUN mkdir -p /src
RUN git clone https://github.com/krepa098/solar-api-bridge.git /src/solar-api-bridge --depth=1

# build backend
WORKDIR /src/solar-api-bridge
RUN cargo b --release
RUN mv target/release/solar-api-bridge solar-api-bridge
RUN rm -rf target

# ===============================================================================
# RUNNER
# ===============================================================================
FROM debian:bullseye-slim AS runner

# copy builds
RUN mkdir -p /usr/local/bin
COPY --from=builder /src/solar-api-bridge/solar-api-bridge /usr/local/bin/solar-api-bridge

# entrypoint
ENTRYPOINT ["/usr/local/bin/solar-api-bridge"]
