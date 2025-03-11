# uv and maturin CLI tools required

setup:
	# or make venv in any other way
	uv venv

test:
	. .venv/bin/activate && \
	maturin develop && \
	python test.py
