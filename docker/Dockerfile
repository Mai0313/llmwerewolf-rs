FROM rust:1.89.0-alpine AS builder

LABEL maintainer="Wei Lee <mai@mai0313.com>" \
    org.label-schema.name="llmwerewolf" \
    org.label-schema.vendor="Wei Lee" \
    org.label-schema.schema-version="1.0" \
    com.centurylinklabs.watchtower.stop-signal="SIGINT"

WORKDIR /app
COPY . .
RUN cargo build --release --locked

########################################################################################

FROM alpine:latest AS prod
COPY --from=builder /app/target/release/llmwerewolf /usr/local/bin/llmwerewolf
ENTRYPOINT ["llmwerewolf"]
