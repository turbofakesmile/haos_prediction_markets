FROM oven/bun:1

WORKDIR /app

# Install curl for healthcheck
RUN apt-get update && \
    apt-get install -y curl && \
    rm -rf /var/lib/apt/lists/*

# Copy package files
COPY package.json bun.lockb ./

# Install dependencies
RUN bun install

# Copy source code
COPY . .

# Expose the default port
EXPOSE 3000

# Start the application
CMD ["bun", "run", "src/index.ts"]
