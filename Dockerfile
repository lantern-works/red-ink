# we customize rustcross to include the openssl libraries
# so websockets will work

FROM japaric/arm-unknown-linux-musleabi
ENV PKG_CONFIG_ALLOW_CROSS=1
RUN apt update && DEBIAN_FRONTEND=noninteractive apt install -y curl libssl-dev

# This is the directory where the ARM-compiled OpenSSL will live
ENV INSTALL_DIR "/lib/precompiled-openssl-arm"

# Download and untar OpenSSL
RUN mkdir -p /src/openssl \
    && curl -OLs https://github.com/openssl/openssl/archive/OpenSSL_1_1_0g.tar.gz \
    && tar xzf OpenSSL_1_1_0g.tar.gz -C /src/openssl \
    && mkdir -p $INSTALL_DIR

# Configure and compile OpenSSL for ARM32
RUN cd /src/openssl/openssl-OpenSSL_1_1_0g \
    && ./Configure linux-generic32 shared --prefix=$INSTALL_DIR --openssldir=$INSTALL_DIR/openssl \
    && make depend \
    && make \
    && make install

# Tell the openssl-sys crate where to look for OpenSSL
ENV OPENSSL_DIR=$INSTALL_DIR

# Clean up intermediate directories
RUN rm /src -rf
