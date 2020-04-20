# docker build -t xena/helloworld .
# docker run --rm -itp 8000:8000 xena/helloworld

# Use minimal image
FROM rustlang/rust:nightly-slim AS build

# Where to build
WORKDIR /src/helloworld

# Copy the source code to the container
COPY . .

# Build program in release mode
RUN cargo build --release

# Create runtime image
FROM ubuntu:18.04

# Copy the binary
COPY --from=build /src/helloworld/target/release/helloworld /usr/local/bin/helloworld

# Start service on boot
CMD ["usr/local/bin/helloworld"]
