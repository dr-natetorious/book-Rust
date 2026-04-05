FROM rust:1.94.1-bookworm

RUN apt-get update \
  && apt-get install -y --no-install-recommends \
    bash \
    ca-certificates \
    ruby \
    ruby-dev \
    build-essential \
    git \
    nodejs \
    npm \
    chromium \
    libnss3 \
    libfreetype6 \
    libharfbuzz0b \
    fonts-freefont-ttf \
  && gem install asciidoctor asciidoctor-pdf asciidoctor-epub3 asciidoctor-diagram --no-document \
  && PUPPETEER_SKIP_DOWNLOAD=true npm install -g @mermaid-js/mermaid-cli \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/*

ENV PUPPETEER_EXECUTABLE_PATH=/usr/bin/chromium-browser

WORKDIR /workspace

COPY docker-entrypoint.sh /usr/local/bin/docker-entrypoint.sh
RUN chmod +x /usr/local/bin/docker-entrypoint.sh

ENTRYPOINT ["/usr/local/bin/docker-entrypoint.sh"]
CMD ["help"]
