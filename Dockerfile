FROM node:22-slim

# Install additional development tools and dependencies for Rust
RUN apt-get update && apt-get install -y \
    git \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -s -- -y 

# Ensure app directory exists with proper permissions
# The node:alpine image already has a node user/group
# RUN mkdir -p /app && chown -R node:node /app


# Install dependencies only when needed
# COPY --chown=node:node package*.json ./
# RUN npm install

# Expose the development server port
EXPOSE 4321

# Default command - will be overridden by VS Code
CMD ["npm", "run", "dev", "--", "--host", "0.0.0.0"]