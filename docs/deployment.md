# GitHub Pages 部署指南

项目生成纯静态 Pages artifact，不需要服务端进程。生产输出固定为 `dist/public`，其中
只有预渲染 HTML、当前 hash 资产、`.nojekyll` 和自定义 `404.html`。

## GitHub 仓库配置

1. 将仓库默认分支设为 `main`。
2. 在 Settings → Pages → Build and deployment 中选择 **GitHub Actions**。
3. 推送 `main` 或在 Actions 页面手动运行 **GitHub Pages** workflow。

workflow 使用最小权限：`contents: read`、`pages: write`、`id-token: write`。部署 job
绑定 `github-pages` environment，且只有 build/test job 成功后才运行。

## Workflow 执行内容

`.github/workflows/pages.yml` 固定 Rust 1.91.1、WASM target、Dioxus CLI 0.7.3、
Node 24 和 `package-lock.json` 中的 Playwright 1.62.1。CI 执行：

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
cargo check --target wasm32-unknown-unknown --no-default-features --features web --locked
scripts/build-pages.sh "$BASE_PATH" release
node scripts/verify-pages-artifact.mjs dist/public "$BASE_PATH"
npm run test:e2e -- --project=chromium --project=firefox --project=webkit
```

失败时上传 `test-results` 和 `playwright-report`；成功后由
`actions/upload-pages-artifact` 上传 `dist/public`，再由 `actions/deploy-pages` 发布。

## 构建路径

仓库站点的 base path 是仓库名：

```bash
scripts/build-pages.sh arcademic-rust release
```

生成引用形如 `/arcademic-rust/assets/...`。用户/组织根站点或自定义域使用空 base
path：

```bash
scripts/build-pages.sh "" release
```

生成引用形如 `/assets/...`。构建脚本每次先清理该 profile 的 Dioxus web 输出，避免
把旧 hash JS/WASM/CSS 带入 Pages artifact；随后通过 Rust finalizer 验证配置内容、
规范化根路径 URL，并生成 `404.html`。

## 本地 Pages 等价预览

repository-path：

```bash
node scripts/serve-pages.mjs dist/public 3200 arcademic-rust
```

根路径：

```bash
node scripts/serve-pages.mjs dist/public 3200
```

该服务器复现 Pages 的前缀、尾斜杠和 404 状态。未知路径返回 HTTP 404，同时
`404.html` 保留完整 SSR 内容并用 meta refresh 跳回配置的站点根路径。

## Artifact 契约

```bash
node scripts/verify-pages-artifact.mjs dist/public arcademic-rust
```

verifier 要求：

- `.nojekyll`、`index.html`、`404.html` 存在；
- 恰好一个当前 CSS、JS、WASM 和 favicon，共 7 个文件；
- 没有可执行文件或服务端二进制；
- `index.html` 包含预渲染配置内容；
- 所有本地引用存在并使用正确 base path；
- `404.html` 重定向到正确根路径。

## 自定义域名

自定义域通常部署根路径构建。将域名写入 Pages 的 Custom domain 设置并按 GitHub
给出的 DNS 记录配置。若需要仓库内 `CNAME` 文件，应在 `scripts/build-pages.sh` 的
artifact 组装阶段显式生成，并同步更新 verifier 的文件清单；不要手工修改
`dist/public`，因为该目录每次构建都会重建。
