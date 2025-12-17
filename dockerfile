# Use official Rust image as base
FROM rust:latest

# Install system dependencies
RUN apt-get update && apt-get install -y \
  curl \
  git \
  build-essential \
  pkg-config \
  libssl-dev \
  npm \
  nodejs \
  && rm -rf /var/lib/apt/lists/*

# Install Node.js LTS (recommended for Svelte development)
RUN npm install -g npm@latest

# Install Rust toolchain components
RUN rustup component add rustfmt clippy
RUN rustup update

# Install useful Rust tools
RUN cargo install cargo-watch cargo-edit cargo-outdated

# Set working directory
WORKDIR /workspaces/discordbot

# Expose ports (adjust as needed)
# 3000 for main web server, 3001 for Svelte dev server
EXPOSE 3000 3001

# Default command
CMD ["/bin/bash"]