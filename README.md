<h1>tauri-pure-admin</h1>

**中文** | [English](./README.en-US.md)

- [tauri-pure-admin](https://github.com/xiaoxian521/tauri-pure-admin) 是在 [pure-admin-thin](https://github.com/xiaoxian521/pure-admin-thin) 的基础上开发
- 当然平台还提供 `electron` 版本的 [electron-pure-admin](https://github.com/xiaoxian521/electron-pure-admin)

### 安装依赖

```sh
pnpm install
```

### 初始化

```sh
# 一路回车即可
pnpm tauri
```

### 启动

```sh
# 桌面端
pnpm tauri:dev
```

```sh
# 浏览器端
pnpm dev
```

### 打包

```sh
# 桌面端
pnpm tauri:build
```

```sh
# 浏览器端
pnpm build
```

### 图标生成

```sh
# 修改public文件夹下的app-icon.png图标为实际项目图标，格式为1024x1024px的png，然后执行下面的命令即可一键生成所有平台的icon并放在src-tauri/icons文件夹中
pnpm tauri:icon
```

### 资料

- `tauri` 比 `electron` 更强，[推荐文档](https://www.cnblogs.com/Grewer/p/12789261.html)
- 如果没有安装 `tauri`，请阅读文档 [tauri](https://tauri.app/zh/)
