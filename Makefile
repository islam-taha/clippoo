.PHONY: build rebuild install clean test run-daemon run-ui setup-shortcuts remove-shortcuts

PREFIX ?= $(HOME)/.local

build:
	cargo build --release --workspace

rebuild:
	cargo clean
	cargo build --release --workspace

install: build
	@echo "Installing Clippoo..."
	@if [ ! -f target/release/clippoo-daemon ] || [ ! -f target/release/clippoo-ui ]; then \
		echo "Error: Binaries not found. Running full rebuild..."; \
		$(MAKE) rebuild; \
	fi
	mkdir -p $(PREFIX)/bin
	cp target/release/clippoo-daemon $(PREFIX)/bin/
	cp target/release/clippoo-ui $(PREFIX)/bin/
	cp scripts/clippoo-ui.sh $(PREFIX)/bin/
	chmod +x $(PREFIX)/bin/clippoo-ui.sh
	mkdir -p $(HOME)/.local/share/clippoo/scripts/
	cp scripts/setup-shortcuts.sh $(HOME)/.local/share/clippoo/scripts/
	cp scripts/uninstall-shortcuts.sh $(HOME)/.local/share/clippoo/scripts/
	chmod +x $(HOME)/.local/share/clippoo/scripts/setup-shortcuts.sh
	chmod +x $(HOME)/.local/share/clippoo/scripts/uninstall-shortcuts.sh
	mkdir -p $(HOME)/.config/systemd/user/
	cp systemd/clippoo-daemon.service $(HOME)/.config/systemd/user/
	systemctl --user daemon-reload
	systemctl --user enable clippoo-daemon
	systemctl --user start clippoo-daemon || true
	@echo "Running shortcut setup..."
	@$(HOME)/.local/share/clippoo/scripts/setup-shortcuts.sh || true

uninstall:
	@echo "Removing keyboard shortcuts..."
	@$(HOME)/.local/share/clippoo/scripts/uninstall-shortcuts.sh 2>/dev/null || true
	systemctl --user stop clippoo-daemon || true
	systemctl --user disable clippoo-daemon || true
	rm -f $(PREFIX)/bin/clippoo-daemon
	rm -f $(PREFIX)/bin/clippoo-ui
	rm -f $(PREFIX)/bin/clippoo-ui.sh
	rm -f $(HOME)/.config/systemd/user/clippoo-daemon.service
	rm -rf $(HOME)/.local/share/clippoo/scripts
	systemctl --user daemon-reload

clean:
	cargo clean
	rm -rf $(HOME)/.local/share/clippoo

test:
	cargo test --workspace

run-daemon:
	RUST_LOG=info cargo run --bin clippoo-daemon

run-ui:
	cargo run --bin clippoo-ui

setup-shortcuts:
	@echo "Setting up keyboard shortcuts..."
	@if [ -f $(HOME)/.local/share/clippoo/scripts/setup-shortcuts.sh ]; then \
		$(HOME)/.local/share/clippoo/scripts/setup-shortcuts.sh; \
	else \
		./scripts/setup-shortcuts.sh; \
	fi

remove-shortcuts:
	@echo "Removing keyboard shortcuts..."
	@if [ -f $(HOME)/.local/share/clippoo/scripts/uninstall-shortcuts.sh ]; then \
		$(HOME)/.local/share/clippoo/scripts/uninstall-shortcuts.sh; \
	else \
		./scripts/uninstall-shortcuts.sh; \
	fi

help:
	@echo "Available targets:"
	@echo "  build           - Build the project in release mode"
	@echo "  rebuild         - Clean and rebuild the project"
	@echo "  install         - Install binaries, systemd service, and shortcuts"
	@echo "  uninstall       - Remove all installed files and shortcuts"
	@echo "  clean           - Clean build artifacts and data"
	@echo "  test            - Run all tests"
	@echo "  run-daemon      - Run the daemon in debug mode"
	@echo "  run-ui          - Run the UI in debug mode"
	@echo "  setup-shortcuts - Set up keyboard shortcuts"
	@echo "  remove-shortcuts- Remove keyboard shortcuts"