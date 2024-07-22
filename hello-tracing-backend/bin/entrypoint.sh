#!/bin/bash

trap 'rm /var/run/hello-tracing-backend/running' EXIT

touch /var/run/hello-tracing-backend/running
hello-tracing-backend
