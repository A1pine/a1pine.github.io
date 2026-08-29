# 最终完成审计

审计对象：用户提出的完整 Rust 1:1 仿站目标、`docs/migration-plan.md` 七阶段标准和
`docs/source-audit.md` 模块/行为矩阵。只把直接的代码、生成 artifact 和测试结果视为
完成证据。

## 显式目标

| 要求 | 直接证据 | 结论 |
|---|---|---|
| 仔细检查 Nuxt 项目 | `source-audit.md` 记录页面树、组件、composable、CSS、配置、产物、响应式几何、死代码和源缺陷 | 完成 |
| 评估七个 Rust 框架 | `architecture/adr-001-dioxus.md` 对 Yew、Iced、egui、Dioxus、Leptos、Makepad、Sycamore 加权比较 | 完成 |
| 选择适合的实现 | ADR 选择 Dioxus 0.7.3，并记录 SSG/CLI 风险与重新评估触发条件 | 完成 |
| 1:1 模块与视觉 | Phase 6 固定 Chromium 十状态差异 0–0.3638%，六个 layout landmark 桌面/移动差异 ≤2 px | 完成 |
| 所有功能、动画、切换特效 | `tests/e2e/behavior.spec.ts` 覆盖主题、菜单、键盘、pointer/blinking、进度、hover、tooltip、局部滚动和 ScrollToTop；视觉测试覆盖十个状态 | 完成 |
| 所有信息可配置 | `config/site.toml` + deny-unknown typed `SiteConfig`；替换配置 SSR 测试和源字符串扫描；`configuration.md` | 完成 |
| 生成静态网页 | root/repository release 均为预渲染 `index.html` + 静态资产，无服务端二进制 | 完成 |
| GitHub Pages | workflow、`.nojekyll`、base path、custom 404、Pages 等价预览、deployment guide | 完成 |
| 详细阶段计划与验收 | `migration-plan.md` 和 Phase 1–7 acceptance records | 完成 |
| 单元测试设计 | 35 个 Rust 单测覆盖配置、模型、finalizer；替换配置 integration；Playwright E2E/axe/visual | 完成 |
| 每阶段 Git commit | Phase 0–7 均使用独立 `phase N:` commit；Phase 7 commit 记录见最终仓库日志 | 完成 |

## 模块矩阵

| 模块 | 内容/状态/运动证据 | 验收证据 | 结论 |
|---|---|---|---|
| Background | pointer RAF、coarse resize、visibility pause、blinking grid、主题背景 | 三浏览器 pointer/blink，桌面明暗视觉 | 完成 |
| Navbar | 六链接、About anchor、主题持久化、移动菜单、CV、阅读进度 | 键盘/重载/Escape/hover/scroll E2E，menu/CV 视觉 | 完成 |
| Hero | 图片与占位、状态时区、联系信息、简介、兴趣、social | work-hour 单测、媒体几何、SSR、desktop/mobile 视觉 | 完成 |
| News | 三条时间线、标签映射、fallback、stagger、hover | tag 单测、内容/hover E2E、landmark/像素比较 | 完成 |
| Research | totals、五年归一化 bars、入场、tooltip | normalization 单测、tooltip E2E/视觉、响应式局部滚动 | 完成 |
| Publications | 三卡片、图片、作者高亮、标签、可配置 actions、hover | action/link 单测、blocked-image 几何、hover 视觉 | 完成 |
| Teaching | 四课程、1/2/4 列、固定标题、stagger/hover | 配置顺序、三浏览器 hover、desktop/mobile landmark | 完成 |
| Activity | 52×7 deterministic grid、标签、legend、入场、canvas hover、主题、键盘滚动 | golden/范围/364 单测与 E2E、tooltip 0% pixel diff、no-JS DOM | 完成 |
| Footer | 模板、build/fixed year、主题 | 模板单测、SSR/视觉 | 完成 |
| ScrollToTop | strict threshold、smooth/reduced motion、hover | 300/301 单测与 E2E、视觉状态 | 完成 |

## 配置与静态交付

- profile 文案、链接、图像、标签、统计、课程、热图、状态、主题、背景、动画、页脚、
  SEO 和 accessible labels 均来自 TOML；布局实现细节留在 CSS/Rust。
- Serde 拒绝未知字段；验证 required strings、URL、唯一 ID/anchor、范围、内部链接、
  动画持续时间和 footer placeholders。
- finalizer 结构化设置 `lang`/主题变量、规范化根资源 URL、验证所有配置值和 base
  path，并生成 404。
- artifact verifier 要求精确 7 文件清单、存在的 base-path 资源、SSR token、404
  目标、无可执行文件。

## 自动化证据

- Rust：35 unit + 1 integration，format、Clippy、WASM check。
- Browser：Chromium/Firefox/WebKit 对 root 与 repository artifact 各 13 passed、2
  expected skips；console/page error/首方失败请求为 0。
- Accessibility：Chromium 桌面/移动、浅/深 axe 0 violations。
- Lighthouse：两套路径 Accessibility 100、SEO 100、Best Practices 100。
- Visual：十状态全部 ≤0.5%，六个 landmark 全部 ≤2 CSS px；测试无任意 sleep。
- Reproducibility：连续两次清理式 repository release 的 7 文件 SHA-256 全部一致。
- Clean clone：从 Phase 7 commit 的新 `git clone --no-local` 在无 target/dist/node_modules
  状态下通过 npm、Rust、WASM、release、artifact verifier 和三浏览器 CI 等价命令。

## 残余边界

远程 Unsplash 可用性属于外部网络状态；页面已通过固定宽高、背景占位、responsive
srcset 和 blocked-image E2E 保证其失败不会移动布局。视觉门槛按计划屏蔽远程图片与
随机 blinking cell。该边界不构成缺失功能或间接证据。

审计未发现未实现模块、未配置资料、未验证交互或部署缺口。
