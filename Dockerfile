# Use the official image as a parent image.
FROM rust:1.48.0-alpine AS builder

# Set the working directory.
WORKDIR /usr/src/wasm-parser

# Copy the files from your host to your current location.
COPY . .

# Add metadata to the image to describe which port the container is listening on at runtime.
EXPOSE 8080


# Install the OpenCL dev headers
RUN ["apk", "update"]
RUN ["apk", "add", "opencl-icd-loader-dev"]
RUN ["apk", "add", "--no-cache", "musl-dev"]

# Run the specified command within the container.
RUN [ "cargo", "build" ]

# copy the finished binary back
FROM scratch AS export-stage
COPY --from=builder /usr/src/wasm-parser/target/debug/wasm-parser /