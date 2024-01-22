#!/usr/bin/env bash

set -o errexit

make postgres

cargo shuttle run