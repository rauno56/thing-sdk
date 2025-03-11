#!/usr/bin/sh

source .venv/bin/activate
maturin develop
python test.py
