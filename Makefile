VENV_DIR=.venv
ifeq ($(OS),Windows_NT)
	VENV_BIN=$(VENV_DIR)/Scripts
else
	VENV_BIN=$(VENV_DIR)/bin
endif

.PHONY: venv
venv: requirements

.PHONY: cleanvenv
cleanvenv:
	rm -rf $(VENV_DIR)

$(VENV_DIR):
	python3 -m venv $(VENV_DIR)

.PHONY: requirements
requirements: $(VENV_DIR) requirements-dev.txt
	$(VENV_BIN)/pip install -r requirements-dev.txt

.PHONY: format
format:
	$(VENV_BIN)/black .

.PHONY: test
test:
	$(VENV_BIN)/python -m pytest

.PHONY: run
run:
	$(VENV_BIN)/python plump/game.py