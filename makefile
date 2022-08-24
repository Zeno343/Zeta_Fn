BIN = target/zeta_fn.hex
LOADER = teensy_loader_cli/teensy_loader_cli

$(LOADER):
	git submodule update --init
	$(MAKE) -C teensy_loader_cli

$(BIN):
	cargo objcopy --release -- -O ihex $@

.PHONY: flash
flash: $(BIN) $(LOADER)
	$(LOADER) --mcu=TEENSY41 -w $<
