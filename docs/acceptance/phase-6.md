# Phase 6 验收记录

验收日期：2026-08-29。Rust 对象为 repository-path release SSG：
`http://127.0.0.1:3200/arcademic-rust/`；Nuxt 对照为重新生成的静态产物：
`http://127.0.0.1:3300/`。

## E2E 架构

- Playwright 1.62.1 固定 Chromium 151，并安装 Firefox 153、WebKit 26.5。
- 每个测试使用独立 browser context；定位优先使用 role、name、label 和可见文本。
- `trace: retain-on-failure`、`screenshot: only-on-failure`，并保存直接的
  source/Rust/diff 裁剪图。
- 测试中无 `waitForTimeout`、`setTimeout` 或自定义 sleep；动画等待均基于可观察的
  DOM、canvas 像素或滚动状态。
- Unsplash 图片由有效的透明 SVG fixture 替换；随机 blinking grid 与 pointer glow
  在视觉比较中屏蔽。

## 跨浏览器行为

Chromium、Firefox、WebKit 均通过以下四个独立用例：

- 全模块内容、10 个 article、3+3 个论文操作、364 个 gridcell 和媒体占位尺寸。
- 主题持久化、深色 canvas 配置色、pointer glow、blinking grid、News/Teaching
  hover、柱图 tooltip、Activity tooltip、阅读进度和 ScrollToTop。
- 移动菜单键盘操作、Escape 焦点返回、主题按钮、柱图/Activity 局部滚动、页面无
  横向溢出和文本容器溢出计数为 0。
- 禁用 JavaScript 时五个模块、364 个 gridcell 和有效外链仍存在于 SSR HTML。

每个项目的 browser console、page error 和首方失败请求数组均为空。axe 在固定
Chromium 的桌面/移动、浅色/深色四种状态为 0 violations；Firefox/WebKit 项目按
设计跳过重复 axe 扫描。

## 视觉结果

Pixelmatch 使用 `threshold = 0.2`、`includeAA = false`。每个差异比例均低于计划的
0.5% 上限：

| 状态 | 稳定裁剪 | 差异 |
|---|---:|---:|
| Desktop light | 1440×1000 | 0.2668% |
| Desktop dark | 1440×1000 | 0.1331% |
| Mobile light | 390×844 | 0.0896% |
| Mobile dark | 390×844 | 0.0656% |
| Mobile menu | 390×844 | 0.0687% |
| CV hover | 1440×1000 | 0.1582% |
| Publication hover | 960×339 | 0.3638% |
| Chart tooltip | 629×306 | 0.0878% |
| Heatmap hover | 150×80 | 0.0000% |
| ScrollToTop | 1440×1000 | 0.2361% |

`nav`、`#home`、`#news`、`#publications`、`#teaching`、`#activity` 在 1440 px 与
390 px 视口的文档 top/height 差异均不超过 2 CSS px。全部视觉测试先激活
viewport 动画，再执行状态交互和截图。

Activity 在 SSR 中保留完整 DOM 网格；正常动画结束后由 Rust/WASM canvas 使用与
Nuxt 相同的 `roundRect` 算法接管绘制。减少动态效果时立即切换。主题 MutationObserver
确保 canvas 在 light/dark/system 切换后使用 TOML 中对应的五级颜色。
