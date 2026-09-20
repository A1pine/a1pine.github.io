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

`[site]` 控制标题、描述、语言、canonical URL、OpenGraph/Twitter 图片、robots 和
跳转到正文的标签。社交图片应使用可公开访问的绝对 HTTPS URL。`base_path` 通常
保留为空，GitHub Pages 的仓库路径由构建脚本参数注入。

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
google_site_verification = "your-google-search-console-token"
skip_to_content_label = "Skip to main content"
base_path = ""
```

`google_site_verification` 是 Google Search Console 提供的验证令牌。构建后的根页面只
生成一个 `<meta name="google-site-verification">`，令牌不会随中英文切换改变。

`[navbar]` 设置品牌、CV、移动菜单和导航链接。站内链接必须指向已有的栏目
锚点，例如 `#experience`；外部链接应使用完整 HTTPS URL。

颜色模式只跟随操作系统的 `prefers-color-scheme`。站点不提供手动覆盖，也不会读取
或写入 localStorage、sessionStorage、Cookie；系统主题在页面打开期间变化时会立即
同步。

## 自动语言匹配

英文文案由 `config/site.toml` 提供，同时也是 SSG/SSR 和缺失翻译时的回退内容。
简体中文翻译位于 `config/locales/zh-CN.json`，使用按功能命名的稳定翻译键。构建脚本
会校验 JSON 语法，单元测试会校验所有必需翻译键均存在且非空。

浏览器端按 `navigator.languages` 的优先顺序协商 `zh-CN` 或英文；不支持的语言回退
英文。页面监听 `languagechange`，因此操作系统或浏览器语言在页面打开期间发生变化
时，会同步更新正文、无障碍标签、动态 GitHub 文案、页面标题、分享元数据以及
`html[lang]`。语言偏好不会写入 localStorage、sessionStorage 或 Cookie。

新增当前可见文案时，应同时：

1. 保留 `site.toml` 中的英文回退内容；
2. 在 `zh-CN.json` 中新增对应翻译键；
3. 将该键加入 `src/localization.rs` 的 `REQUIRED_ZH_CN_KEYS`；
4. 为动态模板使用具名占位符，例如 `{total}`，不要拼接依赖语序的翻译片段。

## 内容栏目

- `[hero]`：姓名、职位、地点、单位、简介、头像、在线状态、兴趣和社交链接。
- `[news]`：栏目标题、时间线条目和标签配色。
- `[publications]`：统计数字、年度柱图、论文卡片、作者高亮和 PDF/代码操作。
- `[teaching]`：课程卡片和资料链接。
- `[activity]`：VibeUsage 徽章、GitHub 用户名、公开 contribution API、资料链接、
  热图尺寸和五级颜色。`vibe_badge_url` 保持远程引用以展示近 7 日实时数据，
  `vibe_profile_url` 是点击徽章后的目标页面。
- `[footer]`：所有者、平台名、年份策略和文本模板。

`[news]`、`[publications]`、`[teaching]` 和 `[activity]` 支持 `enabled`。设为
`false` 时，该栏目不会出现在生成页面中；导航中也不应保留指向该栏目的链接。
论文的 `stats`/`items` 与课程的 `items` 未配置时默认为空数组，因此无需添加占位内容。
Activity 在 SSR 阶段输出空白加载网格，浏览器加载后通过 `api_url` 获取最近一年的
实时 contribution 日期、次数与等级。该接口必须允许跨域请求，且不应在前端 URL
或请求头中放置 GitHub access token。

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

`[[footer.technologies]]` 按顺序配置页脚技术名称及官方图标。`icon` 支持 `rust`、
`vue`、`dioxus`；列表为空时回退显示 `powered_by` 纯文本。

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
