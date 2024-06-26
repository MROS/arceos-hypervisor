# arceos-hypervisor

## 預備

### 設定分支
```
git checkout hypervisor
git submodule update --init --recursive
```

## 執行各種 arceos

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

### 同時執行多個虛擬機
```sh
# 編譯第二個虛擬機的二進位檔
make ARCH=riscv64 A=apps/fs/shell LOG=debug MODE=debug VM=2 build

# GUEST2 將被載入至 0xa020_0000 啟動，相應的設備樹文件被載入到 0xa000_0000
make A=apps/hv ARCH=riscv64 HV=y LOG=debug MODE=debug GUEST=shell GUEST2=shell FS=y justrun
```

## 執行 nimbos 虛擬機
參照[此段落](https://github.com/MROS/hypervisor-blog/blob/main/%E5%9C%A8arceos-hypervisor%E4%B8%8A%E5%9F%B7%E8%A1%8C%E5%90%84%E7%A8%AEOS.md#%E5%9F%B7%E8%A1%8C-nimbos)以編譯及設定設備樹。

若想要同時執行兩個 nimbos ，需修改 `kernel/platforms/qemu-virt-riscv.toml` ，將物理地址起始位置修改到 `0xa000_0000` ，再編譯一份二進位檔，丟至 `apps/hv/guest/nimbos` ，命名為 nimbos-2.bin 。

執行
```sh
make A=apps/hv ARCH=riscv64 HV=y LOG=info MODE=debug GUEST=nimbos GUEST2=nimbos run
```

### 驗證兩個 nimbos 同時執行
使用反引號 \` 可以切換輸入會被導向哪個虛擬機，可以迅速輸入 `sleep` 之後按 \` 切換輸入，再另一臺虛擬機上也執行 `sleep` ，可以看到兩個程序交替輸出。

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

