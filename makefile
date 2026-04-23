APP_ID=com.github.rrebeiz.music_info
SERVICE_PATH=$(HOME)/.config/systemd/user/music_info.service

## build: builds the app
build:
	@echo "building..."
	cargo build --release
	@echo "built!"

## dev: runs dev mode
dev:
	@echo "running dev"
	cargo run

## install: installs binary + systemd service
install:
	@echo "installing binary..."
	cargo install --path . --force

	@echo "creating systemd service..."
	mkdir -p $(HOME)/.config/systemd/user

	cp music-info.service $(SERVICE_PATH)
	systemctl --user daemon-reload
	systemctl --user enable --now music_info
	@echo "installed and started!"

## install-flatpak: installs flatpak + systemd service
install-flatpak:
	@echo "building flatpak..."
	flatpak-builder build-dir flatpak/$(APP_ID).json --install --user

	@echo "creating systemd service..."
	mkdir -p $(HOME)/.config/systemd/user

	cp ./flatpak/music-info-flatpak.service $(SERVICE_PATH)
	systemctl --user daemon-reload
	systemctl --user enable --now music_info

	@echo "flatpak installed and started!"

## uninstall: uninstalls the service and binary
uninstall:
	@echo "stopping service..."
	systemctl --user disable --now music_info || true

	@echo "removing service file..."
	rm -f $(SERVICE_PATH)
	systemctl --user daemon-reload

	@echo "removing binary..."
	cargo uninstall music_info || true

	@echo "uninstalled!"


## uninstall-flatpak: uninstalls flatpak + systemd service
uninstall-flatpak:
	@echo "stopping service..."
	systemctl --user disable --now music_info || true

	@echo "removing service file..."
	rm -f $(SERVICE_PATH)

	systemctl --user daemon-reload

	@echo "uninstalling flatpak..."
	flatpak uninstall -y --noninteractive $(APP_ID) || true

	@echo "flatpak uninstalled!"

## install-bin: downloads the prebuilt binary and installs it
install-bin:
	@echo "downloading prebuilt binary..."
	curl -L https://github.com/rrebeiz/music_info/releases/latest/download/music_info -o music_info

	chmod +x music_info
	sudo install -Dm755 music_info /usr/local/bin/music_info
	rm music_info

	mkdir -p $(HOME)/.config/systemd/user
	cp music-info.service $(SERVICE_PATH)

	systemctl --user daemon-reload
	systemctl --user enable --now music_info

## uninstall-bin: removes the binary and service file
uninstall-bin:
	@echo "stopping service..."
	systemctl --user disable --now music_info || true

	@echo "removing service file..."
	rm -f $(SERVICE_PATH)
	systemctl --user daemon-reload

	@echo "removing binary (requires sudo)..."
	sudo rm -f /usr/local/bin/music_info

	@echo "uninstall complete!"
