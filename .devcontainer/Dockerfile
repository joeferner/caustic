FROM mcr.microsoft.com/devcontainers/rust:2.0.3-trixie

RUN apt-get update

# install wasm pack and flamegraph
RUN apt-get install -y linux-perf
RUN rustup target add wasm32-unknown-unknown && cargo install wasm-pack
RUN cargo install flamegraph
RUN rm -rf /usr/local/cargo/registry

# install terraform
RUN apt-get install -y gnupg
RUN curl -s https://apt.releases.hashicorp.com/gpg | gpg --dearmor > /usr/share/keyrings/hashicorp-archive-keyring.gpg
RUN echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/hashicorp-archive-keyring.gpg] https://apt.releases.hashicorp.com $(grep -oP '(?<=UBUNTU_CODENAME=).*' /etc/os-release || lsb_release -cs) main" > /etc/apt/sources.list.d/hashicorp.list
RUN apt update
RUN apt-get install terraform

# install node
USER vscode
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash
USER root
