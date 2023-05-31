<h1>tauri-pure-admin</h1>

**English** | [中文](./README.md)

- [tauri-pure-admin](https://github.com/xiaoxian521/tauri-pure-admin) is based on [pure-admin-thin](https://github.com/xiaoxian521/pure-admin-thin) based on the development.
- Of course, the platform also provides the `electron` version of [electron-pure-admin](https://github.com/xiaoxian521/electron-pure-admin)

<p align="center">
  <img alt="electron" width="100%" src="https://yiming_chang.gitee.io/pure-admin-doc/img/tauri/1.jpg">
</p>

### Install dependencies

```sh
pnpm install
```

### start up

```sh
# Desktop
pnpm dev
```

```sh
# Browser
pnpm browser:dev
```

### Pack

```sh
# Desktop
pnpm build
```

```sh
# Browser
pnpm browser:build
```

### Icon Generation

```sh
# Modify the app-icon.png icon under the public folder to the actual project icon, the format is 1024x1024px png, and then execute the following command to generate icons for all platforms with one click and put them in the src-tauri/icons folder
pnpm icon
```

### material

- The performance and package size of `tauri` are much better than `electron`, but the ecology of `electron` is stronger, which means that it is convenient and fast to develop. At present, it is recommended to use `electron` to develop desktop applications [recommended Document](https://www.cnblogs.com/Grewer/p/12789261.html)
- When you plan to use `tauri` to develop desktop applications, you must first have a certain [rust](https://www.rust-lang.org/) language foundation
- If `tauri` is not installed, please read the documentation [tauri](https://tauri.app/)
