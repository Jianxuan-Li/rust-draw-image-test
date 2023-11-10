FROM rust:1.73.0 as builder

RUN apt-get update \
    && apt-get -y install curl build-essential clang pkg-config libjpeg-turbo-progs libpng-dev \
    && rm -rfv /var/lib/apt/lists/*

ENV MAGICK_VERSION 7.1

RUN curl https://imagemagick.org/archive/ImageMagick.tar.gz | tar xz \
    && cd ImageMagick-${MAGICK_VERSION}* \
    && ./configure --with-magick-plus-plus=no --with-perl=no \
    && make \
    && make install \
    && cd .. \
    && rm -r ImageMagick-${MAGICK_VERSION}*

RUN mkdir -p /app
WORKDIR /app