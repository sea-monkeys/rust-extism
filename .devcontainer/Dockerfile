FROM --platform=$BUILDPLATFORM ubuntu:22.04


LABEL maintainer="@k33g_org"

ARG TARGETOS
ARG TARGETARCH

ARG RUST_VERSION=${RUST_VERSION}
ARG EXTISM_VERSION=${EXTISM_VERSION}
ARG NODE_MAJOR=${NODE_MAJOR}


ARG USER_NAME=${USER_NAME}

ARG DEBIAN_FRONTEND=noninteractive

ENV LANG=en_US.UTF-8
ENV LANGUAGE=en_US.UTF-8
ENV LC_COLLATE=C
ENV LC_CTYPE=en_US.UTF-8

# ------------------------------------
# Install Tools
# ------------------------------------
RUN <<EOF
apt-get update 
apt-get install -y curl wget jq git build-essential xz-utils bat software-properties-common sudo sshpass unzip
ln -s /usr/bin/batcat /usr/bin/bat

apt-get clean autoclean
apt-get autoremove --yes
rm -rf /var/lib/{apt,dpkg,cache,log}/
EOF


# ------------------------------------
# Install Extism CLI
# ------------------------------------
RUN <<EOF
wget https://github.com/extism/cli/releases/download/v${EXTISM_VERSION}/extism-v${EXTISM_VERSION}-linux-${TARGETARCH}.tar.gz
    
tar -xf extism-v${EXTISM_VERSION}-linux-${TARGETARCH}.tar.gz -C /usr/bin
rm extism-v${EXTISM_VERSION}-linux-${TARGETARCH}.tar.gz
    
#extism --version
EOF

# ------------------------------------
# Install NodeJS
# ------------------------------------
RUN <<EOF
apt-get update && apt-get install -y ca-certificates curl gnupg
curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key | gpg --dearmor -o /etc/apt/keyrings/nodesource.gpg
echo "deb [signed-by=/etc/apt/keyrings/nodesource.gpg] https://deb.nodesource.com/node_$NODE_MAJOR.x nodistro main" | tee /etc/apt/sources.list.d/nodesource.list
apt-get update && apt-get install nodejs -y
EOF


# ------------------------------------
# Install Rust and WASM dependencies
# ------------------------------------
RUN <<EOF
# Additional dependencies for Rust
apt-get update
apt-get install -y pkg-config libssl-dev clang lld cmake

# Clean up
apt-get clean autoclean
apt-get autoremove --yes
rm -rf /var/lib/{apt,dpkg,cache,log}/
EOF

# ------------------------------------
# Create a new user
# ------------------------------------
# Create new regular user `${USER_NAME}` and disable password and gecos for later
# --gecos explained well here: https://askubuntu.com/a/1195288/635348
RUN adduser --disabled-password --gecos '' ${USER_NAME}

#  Add new user `${USER_NAME}` to sudo group
RUN adduser ${USER_NAME} sudo

# Ensure sudo group users are not asked for a password when using 
# sudo command by ammending sudoers file
RUN echo '%sudo ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers



# Set the working directory
WORKDIR /home/${USER_NAME}

# Set the user as the owner of the working directory
RUN chown -R ${USER_NAME}:${USER_NAME} /home/${USER_NAME}

# Switch to the regular user
USER ${USER_NAME}

# Avoid the message about sudo
RUN touch ~/.sudo_as_admin_successful

# ------------------------------------
# Install OhMyBash
# ------------------------------------
RUN <<EOF
bash -c "$(curl -fsSL https://raw.githubusercontent.com/ohmybash/oh-my-bash/master/tools/install.sh)"
EOF



# ------------------------------------
# Install Rust
# ------------------------------------
RUN <<EOF
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain ${RUST_VERSION}
. $HOME/.cargo/env

# Install WebAssembly target
rustup target add wasm32-wasi
rustup target add wasm32-unknown-unknown

# Install helpful Rust tools
cargo install wasm-pack --force
cargo install wasm-tools
cargo install wasm-bindgen-cli
cargo install cargo-generate
EOF

# Add cargo bin to PATH
ENV PATH="/home/${USER_NAME}/.cargo/bin:${PATH}"
