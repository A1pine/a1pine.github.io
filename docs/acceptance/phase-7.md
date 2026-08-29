# Phase 7 验收记录

验收日期：2026-08-29。

## 构建与 artifact

- 根路径和 `/arcademic-rust/` 均通过清理式 Dioxus release SSG 与 Rust finalizer。
- 连续两次 repository-path 构建的全部 7 个文件 SHA-256 完全一致。
- 从 Phase 7 commit 创建的 `git clone --no-local` 初始状态为空；不复用仓库的
  `target/dist/node_modules`，从零通过 `npm ci`、Rust 四门槛、release SSG、artifact
  verifier 和 Chromium/Firefox/WebKit E2E。
- 两套 artifact 均恰好包含 `.nojekyll`、`index.html`、`404.html`、一个 CSS、一个
  JS、一个 WASM 和一个 favicon；无可执行文件或服务端二进制。
- `index.html` 含完整 SSR 内容；根路径引用为 `/assets/...`，仓库路径引用为
  `/arcademic-rust/assets/...`，所有本地引用都存在。
- 自定义 `404.html` 保留 SSR 内容，返回 HTTP 404，并分别重定向到 `/` 与
  `/arcademic-rust/`。

## Pages 等价预览

`scripts/serve-pages.mjs` 分别在 `http://127.0.0.1:3211/` 和
`http://127.0.0.1:3212/arcademic-rust/` 提供两套 artifact：

- 站点根返回 200；仓库路径缺少尾斜杠时返回 301。
- 未知路径返回带自定义 HTML 的 404。
- CSS、JS、WASM 请求均返回 200，首方失败请求为 0。
- Chromium、Firefox、WebKit 对每套 artifact 均为 13 passed、2 skipped；两个
  skip 是非 Chromium 项目中按设计跳过的重复 axe 用例。
- 固定 Chromium 对每套 artifact 均为 11/11 视觉门槛通过。

## 质量门槛

- `cargo fmt`、Clippy `-D warnings`、35 个单元测试、1 个替换配置集成测试、WASM
  web check 全部通过。
- root 与 repository Lighthouse 均为 Accessibility 100、SEO 100、Best Practices
  100。
- GitHub Actions 使用 Pages 所需最小权限、Rust/npm 缓存、固定工具链、artifact
  verifier、三浏览器 release E2E、失败 diagnostics、Pages artifact upload 和
  environment deployment。

完整目标和 source-audit 矩阵的证据映射见 `final-audit.md`。
