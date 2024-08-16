# Telegraf Plugin for AIDA64

This is a telegraf plugin for AIDA64.

It reads data from AIDA64 and pass them to telegraf.

---

这是一个 telegraf 的插件，从 AIDA64 读取数据，写入 telegraf。

## Usage

0. open AIDA64 > Preferences > hardware monitoring > External Applications > Allow shared memory.
1. Put telegraf-plugin-aida64.exe to, e.g., `C:\Program Files\telegraf`.
2. Copy the content in [example-telegraf.toml](example-telegraf.toml) and append to your telegraf config.
3. Restart your telegraf service.

---

0. 打开 AIDA64 > 设置 > 硬件监视工具 > 外部程序 > 允许共享内存
1. 把 telegraf-plugin-aida64.exe 放到比如 `C:\Program Files\telegraf` 下面。
2. 把 [example-telegraf.toml](example-telegraf.toml) 的内容拷贝到你的 telegraf 设置中。
3. 重启你的 telegraf 服务。

## Metrics format
```
aida64,tag=GPU TGPU1=80 1723823901992998600
           ^   ^     ^value
            `   `key
              `human readable description
```

meaning: **T**emperature of **GPU1** is 80 degree Celsius.

## License

MIT.

