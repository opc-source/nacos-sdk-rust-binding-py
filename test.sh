#!/bin/bash

set -e

# Setup virtualenv:
python3 -m venv venv
# Activate venv:
source venv/bin/activate

# Install maturin:
pip install maturin
pip install behave

# Build bindings:
maturin develop

# Run some tests:
behave tests
