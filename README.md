# arceos-hypervisor

## 預備

### 設定分支
```
git checkout hypervisor
git submodule update --init --recursive
```

## 將 arceos 上的 APP 當虛擬機跑

### 編譯 arceos APP

以 shell 爲例，若想要跑其他 APP ，自行替換路徑即可。

```sh
# VM=n 會使 shell 的二進制檔的 BASE_ADDRESS 增加 0x1000_0000 * n
# 在 apps/fs/shell 目錄生成的二進制檔，會被複製到 apps/hv/guest/shell 下
make ARCH=riscv64 A=apps/fs/shell LOG=debug MODE=debug VM=1 build
# 若發生問題可嘗試清空快取重新建置
# make ARCH=riscv64 A=apps/fs/shell clean
# 編譯 hypervisor
make A=apps/hv ARCH=riscv64 HV=y LOG=debug MODE=debug build

# 在 hypervisor 上執行 shell
# 注意此處用 justrun ，若用 run 會因爲 FS=y 而導致編譯 hypervisor 時注入 fs 模組而出錯
# 在執行不同 APP 時需根據使用到的模組開啓 FS,NET... 等等參數
make A=apps/hv ARCH=riscv64 HV=y LOG=debug MODE=debug GUEST=shell FS=y justrun
```

## 執行 Linux 虛擬機

以下是執行 RISC-V Linux 虛擬機的方法。

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

