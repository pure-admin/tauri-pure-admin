<h1>tauri-pure-admin</h1>

**English** | [中文](./README.md)

- [tauri-pure-admin](https://github.com/xiaoxian521/tauri-pure-admin) is based on [pure-admin-thin](https://github.com/xiaoxian521/pure-admin-thin) based on the development.
- Of course, the platform also provides the `electron` version of [electron-pure-admin](https://github.com/xiaoxian521/electron-pure-admin)

### Install dependencies

```sh
pnpm install
```

### Initialization

```sh
# press enter all the way
pnpm tauri
```

### start up

```sh
# Desktop
pnpm tauri:dev
```

```sh
# Browser
pnpm dev
```

### Pack

```sh
# Desktop
pnpm tauri:build
```

```sh
# Browser
pnpm build
```

### Icon Generation

```sh
# Modify the app-icon.png icon under the public folder to the actual project icon, the format is 1024x1024px png, and then execute the following command to generate icons for all platforms with one click and put them in the src-tauri/icons folder
pnpm tauri:icon
```

### material

- `tauri` is stronger than `electron`, [recommended documentation](https://www.cnblogs.com/Grewer/p/12789261.html)
- If `tauri` is not installed, please read the documentation [tauri](https://tauri.app/)
