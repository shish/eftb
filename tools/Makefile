.DEFAULT_GOAL := check
.PHONY: check lint format format-check typecheck

check: lint format-check typecheck

lint:
	uv run --project . ruff check .

format:
	uv run --project . ruff format .

format-check:
	uv run --project . ruff format --check .

typecheck:
	uv run --project . mypy .
