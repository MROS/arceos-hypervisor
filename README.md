# arceos-hypervisor

## 執行虛擬機

以下是執行 RISC-V Linux 虛擬機的方法。

### 設定分支
```
git checkout hypervisor
git submodule update --init --recursive
```

### 下載 Linux

[點此](https://drive.google.com/drive/folders/1rvcl-fddQSR-Ro1o9Mg3gsJ0A7bXg-Uh)下載 rootfs.img 以及 linux.bin，將此二檔案置於 `apps/hv/guest/linux/`。

### 建置並執行

```
make ARCH=riscv64 A=apps/hv HV=y LOG=info run
```

### 除錯

```
make ARCH=riscv64 A=apps/hv HV=y LOG=debug MODE=debug debug
```

