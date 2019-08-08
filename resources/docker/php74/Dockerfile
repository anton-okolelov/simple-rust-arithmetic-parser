FROM php:7.4.0alpha3-cli

RUN set -e; \
    apt-get update; \
	apt-get install -y --no-install-recommends zip unzip libffi-dev; \
	docker-php-ext-configure ffi; \
    docker-php-ext-install -j$(nproc) ffi; \
    rm -rf /var/lib/apt/lists/*; \
    rm -rf /tmp/*;

# enable ffi for phpdbg
RUN cp /usr/local/etc/php/php.ini-development /usr/local/etc/php/php.ini; \
    echo "ffi.enable=true" >> /usr/local/etc/php/php.ini

# install xdebug
RUN cd /tmp; \
     curl https://codeload.github.com/xdebug/xdebug/tar.gz/master -o xdebug.tar.gz;\
     tar -xzf xdebug.tar.gz; \
     cd xdebug-master; \
     sh rebuild.sh; \
     docker-php-ext-enable xdebug;


RUN curl https://sh.rustup.rs -sSf > rustup.sh;
RUN sh rustup.sh -y

ENV PATH="/root/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"

RUN cargo install cbindgen

WORKDIR /app
