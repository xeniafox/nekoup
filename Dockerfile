FROM archlinux:latest

ENV HOSTNAME nekoup

RUN useradd -m -s /sbin/bash nekoup

COPY ./target/release/nekoup-backend /home/nekoup/nekoup-backend
COPY ./Rocket.toml /home/nekoup/Rocket.toml
RUN chmod a+rx /home/nekoup/nekoup-backend
RUN chmod a+r /home/nekoup/Rocket.toml
RUN chmod a-w /home/nekoup/Rocket.toml

USER nekoup
WORKDIR /home/nekoup

ENTRYPOINT ["/home/nekoup/nekoup-backend"]
