# ---------------------------------------------------
# 1 - Build Stage
#
# Use official rust image to for application build
# ---------------------------------------------------
FROM rust:1.66.1 as build

# Setup working directory
WORKDIR /usr/src/codefee-works-api
COPY . .
COPY .env.docker .env

# Install dependency (Required by diesel)
RUN apt-get update && apt-get install libpq5 -y

# Build application
RUN cargo install --path .

# ---------------------------------------------------
# 2 - Deploy Stage
#
# Use a distroless image for minimal container size
# - Copy `libpq` dependencies into the image (Required by diesel)
# - Copy application files into the image
# ---------------------------------------------------
FROM gcr.io/distroless/cc-debian11

# libpq related (required by diesel)
COPY --from=build /usr/lib/aarch64-linux-gnu/libpq.so* /usr/lib/aarch64-linux-gnu/
COPY --from=build /usr/lib/aarch64-linux-gnu/libgssapi_krb5.so* /usr/lib/aarch64-linux-gnu/
COPY --from=build /usr/lib/aarch64-linux-gnu/libldap_r-2.4.so* /usr/lib/aarch64-linux-gnu/
COPY --from=build /usr/lib/aarch64-linux-gnu/libkrb5.so* /usr/lib/aarch64-linux-gnu/
COPY --from=build /usr/lib/aarch64-linux-gnu/libk5crypto.so* /usr/lib/aarch64-linux-gnu/
COPY --from=build /usr/lib/aarch64-linux-gnu/libkrb5support.so* /usr/lib/aarch64-linux-gnu/
COPY --from=build /usr/lib/aarch64-linux-gnu/liblber-2.4.so* /usr/lib/aarch64-linux-gnu/
COPY --from=build /usr/lib/aarch64-linux-gnu/libsasl2.so* /usr/lib/aarch64-linux-gnu/
COPY --from=build /usr/lib/aarch64-linux-gnu/libgnutls.so* /usr/lib/aarch64-linux-gnu/
COPY --from=build /usr/lib/aarch64-linux-gnu/libp11-kit.so* /usr/lib/aarch64-linux-gnu/
COPY --from=build /usr/lib/aarch64-linux-gnu/libidn2.so* /usr/lib/aarch64-linux-gnu/
COPY --from=build /usr/lib/aarch64-linux-gnu/libunistring.so* /usr/lib/aarch64-linux-gnu/
COPY --from=build /usr/lib/aarch64-linux-gnu/libtasn1.so* /usr/lib/aarch64-linux-gnu/
COPY --from=build /usr/lib/aarch64-linux-gnu/libnettle.so* /usr/lib/aarch64-linux-gnu/
COPY --from=build /usr/lib/aarch64-linux-gnu/libhogweed.so* /usr/lib/aarch64-linux-gnu/
COPY --from=build /usr/lib/aarch64-linux-gnu/libgmp.so* /usr/lib/aarch64-linux-gnu/
COPY --from=build /usr/lib/aarch64-linux-gnu/libffi.so* /usr/lib/aarch64-linux-gnu/
COPY --from=build /lib/aarch64-linux-gnu/libcom_err.so* /lib/aarch64-linux-gnu/
COPY --from=build /lib/aarch64-linux-gnu/libkeyutils.so* /lib/aarch64-linux-gnu/

# Application files
COPY --from=build /usr/local/cargo/bin/codefee-works-api /usr/local/bin/codefee-works-api
COPY --from=build /usr/src/codefee-works-api/.env /.env

CMD ["codefee-works-api"]