FROM rust:alpine3.22

RUN apk add --no-cache \
    bash \
    ca-certificates \
    ruby \
    ruby-dev \
    build-base \
    git \
  && gem install asciidoctor asciidoctor-pdf asciidoctor-epub3 asciidoctor-diagram --no-document

WORKDIR /workspace

COPY docker-entrypoint.sh /usr/local/bin/docker-entrypoint.sh
RUN chmod +x /usr/local/bin/docker-entrypoint.sh

ENTRYPOINT ["/usr/local/bin/docker-entrypoint.sh"]
CMD ["help"]
