# 配置指南

站点的内容、元数据、外观与动画都由 `config/site.toml` 驱动。Rust
代码通过强类型结构读取该文件，并在构建时拒绝未知字段、重复锚点、无效 URL、
越界数值和不完整的页脚模板。修改配置后必须重新构建，浏览器不会在运行时读取
TOML。

## 快速开始

1. 复制仓库后直接编辑 `config/site.toml`。该文件本身是完整、可构建的示例。
2. 先运行 `cargo test --all-features --locked` 验证配置。
3. 运行 `scripts/build-pages.sh arcademic-rust release` 构建项目路径版本，或省略
   第一个参数构建根路径版本。
4. 部署 `dist/public`，不要部署 `target` 中的服务端可执行文件。

`tests/fixtures/alternate-identity.toml` 是第二个最小身份示例；
`tests/configuration_swap.rs` 演示如何在测试中替换整套身份和栏目内容，并验证默认
资料不会泄漏到 SSR HTML。

## 站点与导航

`[site]` 控制标题、描述、语言、canonical URL、OpenGraph/Twitter 图片、robots、
跳转到正文的标签、默认主题和主题存储键。社交图片应使用可公开访问的绝对 HTTPS
URL。`base_path` 通常保留为空，GitHub Pages 的仓库路径由构建脚本参数注入。

```toml
[site]
title = "Ada Lovelace Research Archive"
description = "Notes, publications, teaching, and activity."
language = "en"
canonical_url = "https://example.org/archive/"
social_image_url = "https://example.org/archive/social-card.jpg"
social_image_alt = "Ada Lovelace Research Archive"
social_card = "summary_large_image"
robots = "index, follow"
skip_to_content_label = "Skip to main content"
default_theme = "system"
theme_storage_key = "archive-color-mode"
base_path = ""
```

`[navbar]` 设置品牌、CV、主题按钮、移动菜单和导航链接。站内链接必须指向已有的栏目
锚点，例如 `#news`；外部链接应使用完整 HTTPS URL。

## 内容栏目

- `[hero]`：姓名、职位、地点、单位、简介、头像、在线状态、兴趣和社交链接。
- `[news]`：栏目标题、时间线条目和标签配色。
- `[publications]`：统计数字、年度柱图、论文卡片、作者高亮和 PDF/代码操作。
- `[teaching]`：课程卡片和资料链接。
- `[activity]`：热图尺寸、标签、确定性种子、阈值和五级颜色。
- `[footer]`：所有者、平台名、年份策略和文本模板。

重复条目使用 TOML 数组表：

```toml
[[news.items]]
date = "Aug 2026"
title = "Archive launched"
description = "The public research archive is now available."
tag = "Announcement"

[[teaching.items]]
code = "MATH-101"
title = "Analytical Engines"
semester = "Fall 2026"
description = "A practical introduction to mechanical computation."
materials_url = "https://example.org/courses/math-101"
```

空的论文或课程 URL 会渲染为禁用操作；站外 HTTP(S) 操作会在新标签页打开并带
`noopener noreferrer`，站内路径保持当前浏览上下文。

## 动画与设计

`[animation]` 中的持续时间单位是毫秒，位移单位是像素。`enabled = false` 会关闭
动画；`respect_reduced_motion = true` 会在用户请求减少动态效果时关闭非必要动画和
平滑滚动。`[design]` 控制两种主题的主色、页面背景、正文文字和玻璃卡片颜色。

修改颜色后应重新运行桌面/移动、明暗四种状态的 axe 与视觉测试，确保文本对比度和
交互状态仍满足验收门槛。

## 页脚年份

`year_mode = "build"` 使用构建年份；`year_mode = "fixed"` 使用 `fixed_year`。
`text_template` 必须且只能使用支持的占位符：`{year}`、`{owner}`、
`{powered_by}`、`{hosted_by}`。

## 验证命令

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
cargo check --target wasm32-unknown-unknown --no-default-features --features web --locked
scripts/build-pages.sh arcademic-rust release
```

构建后的 `index.html` 会经过结构化 finalizer：它设置 `<html lang>` 和首屏主题
变量，并确认配置中的内容及 base-path 资源引用全部存在。任何检查失败都会使构建
退出。
