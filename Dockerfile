# using ubuntu (x86/amd64) to support x86_64 assembly instructions
FROM amd64/ubuntu
# FROM --platform=linux/amd64 rust:1.31

# install gcc (and build essential packages)
RUN apt-get update && apt-get install build-essential -y

# adds project into the container
ADD . $HOME/fcc

# runs a terminal
CMD /bin/bash
