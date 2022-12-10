# Needs a DOCKER_USERNAME set
APPLICATION_NAME ?= tack
GIT_HASH ?= $(shell git log --format="%h" -n 1)
# This uses the cargo-get plugin https://crates.io/crates/cargo-get
VERSION ?= $(shell cargo get version)

build: check-env
	docker build --tag ${DOCKER_USERNAME}/${APPLICATION_NAME}:${GIT_HASH} .
	docker tag  ${DOCKER_USERNAME}/${APPLICATION_NAME}:${GIT_HASH} ${DOCKER_USERNAME}/${APPLICATION_NAME}:latest
	docker tag  ${DOCKER_USERNAME}/${APPLICATION_NAME}:${GIT_HASH} ${DOCKER_USERNAME}/${APPLICATION_NAME}:${VERSION}

push:
	docker push -a ${DOCKER_USERNAME}/${APPLICATION_NAME}

check-env:
ifndef DOCKER_USERNAME
  $(error DOCKER_USERNAME is undefined)
endif