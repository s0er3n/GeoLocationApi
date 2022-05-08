FROM archlinux:base-devel
RUN pacman-key --init
RUN pacman --noconfirm -Syu
RUN pacman  --noconfirm -S rust libarchive pkg-config
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rust.sh
RUN chmod +x ./rust.sh
RUN ./rust.sh -y
ENV PATH="/root/.cargo/bin:${PATH}"


RUN rustup default nightly

ENV ROCKET_ADDRES=0.0.0.0
ENV ROCKET_PORT=8080

ADD . /app

WORKDIR /app





RUN cargo build --release

CMD ["./target/release/geoapi"]
