FROM rust:alpine AS builder
LABEL maintainer="mingcheng<mingcheng@apache.org>"

# https://help.mirrors.cernet.edu.cn/alpine/
RUN sed -i 's#https\?://dl-cdn.alpinelinux.org/alpine#https://mirrors.cernet.edu.cn/alpine#g' /etc/apk/repositories

# Add necessary build dependencies
RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    pkgconfig

# Define the missing environment variable
ENV BUILD_DIR="/build"

# Set rustup mirror to accelerate the build
ENV RUSTUP_UPDATE_ROOT="https://mirrors.cernet.edu.cn/rustup/rustup"
ENV RUSTUP_DIST_SERVER="https://mirrors.cernet.edu.cn/rustup"

# Update the latest stable version of rust toolkit
RUN rustup default stable && rustup override set stable

# Start building the application
COPY . ${BUILD_DIR}
WORKDIR ${BUILD_DIR}

# Build the application
RUN cargo build --release \
    && ./target/release/todo --help \
    && cp target/release/todo /bin/todo

# Stage2
FROM alpine

# https://help.mirrors.cernet.edu.cn/alpine/
RUN sed -i 's#https\?://dl-cdn.alpinelinux.org/alpine#https://mirrors.cernet.edu.cn/alpine#g' /etc/apk/repositories

# Install timezone data and set timezone
RUN apk add --no-cache tzdata
ENV TZ="Asia/Shanghai"

# Copy the binary from the builder stage
COPY --from=builder /bin/todo /bin/todo

# Create a non-root user
RUN addgroup -S app && adduser -S app -G app

# Switch to non-root user
USER app

# Define the command to run the application
ENTRYPOINT ["/bin/todo"]
