.DEFAULT_GOAL := all
.PHONY: all backend frontend check lint format format-check typecheck

all: backend frontend

backend:
	$(MAKE) -C backend

frontend:
	$(MAKE) -C frontend

check lint format format-check typecheck:
	$(MAKE) -C tools $@
