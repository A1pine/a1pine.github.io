# Phase 5 验收记录

验收日期：2026-08-29。对象为 repository-path release SSG：
`http://127.0.0.1:3200/arcademic-rust/`。

## 自动化门槛

- `cargo fmt`、Clippy `-D warnings`、WASM web check 全部通过。
- 原生测试通过：34 个单元测试、1 个替换配置集成测试。
- 替换配置生成的 SSR 页面不含生产身份、新闻、论文、课程或页脚默认值。
- 在 `src`、脚本、主 CSS、构建文件、工作流和替换配置测试中扫描生产资料字符串，
  精确词边界扫描为 0 命中。
- release SSG 和结构化 finalizer 通过；配置值和 `/arcademic-rust/` 资源前缀完整。

## 浏览器与无障碍

Playwright 1.62.1 / Chrome for Testing 151 的隔离上下文验证：

- axe 在桌面浅色、桌面深色、移动浅色、移动深色四种状态均为 0 violations。
- Lighthouse：Accessibility 100、SEO 100、Best Practices 100。
- 跳转链接是第一个键盘焦点并把焦点移到 `main`。
- 键盘可切换主题；移动菜单可用 Enter 打开、Escape 关闭并把焦点返回菜单按钮。
- 柱图与热图滚动区均可获得焦点，热图可用 ArrowRight 横向滚动。
- ScrollToTop 可用键盘触发并到达 `scrollY = 0`。
- 320 CSS px 回流时页面 `scrollWidth = clientWidth = 320`；仅柱图和热图在各自
  命名区域内横向滚动。
- 禁用 JavaScript 后，全部五个内容模块、3 篇论文、4 门课程、364 个热图单元以及
  有效站外链接仍存在于预渲染 HTML。
- 页面 `lang`、description、robots、canonical、OpenGraph、Twitter Card 和 favicon
  元数据均来自配置；浏览器 console error 与 page error 均为 0。

## 截图

- `phase-5-desktop-light.png`
- `phase-5-desktop-dark.png`
- `phase-5-mobile-light.png`
- `phase-5-mobile-dark.png`
