<h1>tauri-pure-admin</h1>

**中文** | [English](./README.en-US.md)

- [tauri-pure-admin](https://github.com/xiaoxian521/tauri-pure-admin) 是在 [pure-admin-thin](https://github.com/xiaoxian521/pure-admin-thin) 的基础上开发
- 当然平台还提供 `electron` 版本的 [electron-pure-admin](https://github.com/xiaoxian521/electron-pure-admin)

<p align="center">
  <img alt="electron" width="100%" src="https://yiming_chang.gitee.io/pure-admin-doc/img/tauri/1.jpg">
</p>

### 安装依赖

```sh
pnpm install
```

### 启动

```sh
# 桌面端
pnpm dev
```

```sh
# 浏览器端
pnpm browser:dev
```

### 打包

```sh
# 桌面端
pnpm build
```

```sh
# 浏览器端
pnpm browser:build
```

### 图标生成

```sh
# 修改public文件夹下的app-icon.png图标为实际项目图标，格式为1024x1024px的png，然后执行下面的命令即可一键生成所有平台的icon并放在src-tauri/icons文件夹中
pnpm icon
```

### 资料

- `tauri` 的性能和打包大小要远优于 `electron` ，但 `electron` 的生态更强，生态强意味着开发起来方便快捷，目前还是推荐使用 `electron` 去开发桌面端应用程序 [推荐文档](https://www.cnblogs.com/Grewer/p/12789261.html)
- 当您打算完全使用 `tauri` 去开发桌面端应用程序时，首先要具备一定的 [rust](https://www.rust-lang.org/zh-CN/) 语言基础
- 如果没有安装 `tauri`，请阅读文档 [tauri](https://tauri.app/zh/)
