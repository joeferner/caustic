FROM mcr.microsoft.com/devcontainers/rust:2.0.6-bookworm

RUN apt-get update

# install wasm pack and flamegraph
RUN \
  apt-get install -y linux-perf \
  && rustup target add wasm32-unknown-unknown && cargo install wasm-pack \
  && cargo install flamegraph \
  && rm -rf /usr/local/cargo/registry

# install java
RUN apt-get install -y openjdk-17-jre

# install terraform
RUN \
  apt-get install -y gnupg \
  && curl -s https://apt.releases.hashicorp.com/gpg | gpg --dearmor > /usr/share/keyrings/hashicorp-archive-keyring.gpg \
  && echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/hashicorp-archive-keyring.gpg] https://apt.releases.hashicorp.com $(grep -oP '(?<=UBUNTU_CODENAME=).*' /etc/os-release || lsb_release -cs) main" > /etc/apt/sources.list.d/hashicorp.list \
  && apt update \
  && apt-get install terraform

# install node and sqlx-cli
USER vscode
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash
RUN cargo install sqlx-cli --root ~/.local
USER root
