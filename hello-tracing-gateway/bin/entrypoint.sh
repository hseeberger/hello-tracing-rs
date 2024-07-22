#!/bin/bash

trap 'rm /var/run/hello-tracing-gateway/running' EXIT

touch /var/run/hello-tracing-gateway/running
hello-tracing-gateway
