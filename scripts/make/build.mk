# Main building script

$(OUT_DIR):
	mkdir -p $@

ifeq ($(APP_LANG), c)
  include ulib/c_libax/build.mk
else
  rust_package := $(shell cat $(APP)/Cargo.toml | sed -n 's/name = "\([a-z0-9A-Z_\-]*\)"/\1/p')
  rust_target_dir := $(CURDIR)/target/$(TARGET)/$(MODE)
  rust_elf := $(rust_target_dir)/$(rust_package)
endif

_cargo_build:
	@printf "    $(GREEN_C)Building$(END_C) App: $(APP_NAME), Arch: $(ARCH), Platform: $(PLATFORM), Language: $(APP_LANG)\n"
ifeq ($(APP_LANG), rust)
	$(call cargo_build,--manifest-path $(APP)/Cargo.toml)
	@cp $(rust_elf) $(OUT_ELF)

else ifeq ($(APP_LANG), c)
	$(call cargo_build,-p libax)
endif

$(OUT_BIN): _cargo_build $(OUT_ELF)
	$(OBJCOPY) $(OUT_ELF) --strip-all -O binary $@
ifeq ($(VM),1)
	mkdir -p apps/hv/guest/$(APP_NAME)
	dtc -I dts -O dtb -o apps/hv/guest/linux/linux.dtb apps/hv/guest/linux/linux.dts
	ln -f apps/hv/guest/linux/linux.dtb apps/hv/guest/$(APP_NAME)/$(APP_NAME)-1.dtb
	@cp $(OUT_BIN) apps/hv/guest/$(APP_NAME)/$(APP_NAME)-1.bin
else ifeq ($(VM),2)
	mkdir -p apps/hv/guest/$(APP_NAME)
	dtc -I dts -O dtb -o apps/hv/guest/linux/linux-2.dtb apps/hv/guest/linux/linux-2.dts
	ln -f apps/hv/guest/linux/linux-2.dtb apps/hv/guest/$(APP_NAME)/$(APP_NAME)-2.dtb
	@cp $(OUT_BIN) apps/hv/guest/$(APP_NAME)/$(APP_NAME)-2.bin
endif 

.PHONY: _cargo_build
