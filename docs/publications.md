# Publications 自动同步

Publications 模块的数据来自 `config/publications.json`。该文件由
`scripts/fetch-publications.mjs` 生成，构建时会覆盖 `config/site.toml` 中
publications 的 `items`、`stats`、`total_value`、`citations_value` 和
`last_updated` 字段。

## 数据来源

脚本优先通过 SerpAPI 的 Google Scholar Author 接口获取数据；未设置
`SERPAPI_KEY` 时回退为 Playwright 直接访问 Google Scholar 个人主页（适合本地
网络，GitHub Actions 的数据中心 IP 会被 Google 拦截）。每次抓取每篇论文的：

- 论文标题
- 合作者姓名（主页列表截断时会进入论文详情页补全）
- 发表年份
- 被引次数
- 期刊/会议名称与 Scholar 论文页链接
- 论文类型（`journal`、`conference`、`preprint` 或 `other`）

同时根据论文计算总数、总被引和近五年年度统计，并把抓取日期写入
`last_updated`，该时间会显示在模块末尾。

类型由出版信息推断：`arXiv`/`preprint` 归为预印本，包含 conference、proceedings、
workshop 或 symposium 的出版信息归为会议，其余有正式出版源的条目归为期刊。

## 主图与人工覆盖

Google Scholar Author API 没有标准论文主图字段。同步器只接受 API 明确返回的
`image_url`、`image`、`thumbnail`，或图片格式的 resource；不会抓取出版商页面并把
站点 Logo 当作论文主图。没有可靠图片时继续使用站内生成的论文封面占位。

`config/publication-overrides.json` 用稳定的 Scholar `citation_id`（生成数据中的
`source_id`）覆盖自动字段。支持：`publication_type`、`description`、`tags`、
`image_url`、`image_alt`、`pdf_url` 和 `code_url`。例如：

```json
{
  "gPmLhlAAAAAJ:d1gkVwhDpl0C": {
    "image_url": "https://example.org/paper-cover.jpg",
    "image_alt": "LLM-CECM graphical abstract",
    "code_url": "https://github.com/example/llm-cecm",
    "tags": ["Energy Markets", "Multi-Agent Systems"]
  }
}
```

同步器在合并覆盖后生成 `code_available`。构建产物把该值写入论文卡片的
`data-code-available`；仅当 `code_url` 非空时显示 Code 按钮。这样默认页面没有禁用
Code 按钮，人工补充仓库链接后则自动显示。

## 定时任务

`.github/workflows/publications.yml` 每天北京时间 05:00（UTC 21:00）运行，
也支持手动触发。工作流读取仓库变量 `SCHOLAR_ID`（或同名 secret）：

```bash
gh variable set SCHOLAR_ID --body "<google-scholar-user-id>"
gh secret set SERPAPI_KEY --body "<serpapi-api-key>"
```

SerpAPI 免费套餐每月 100 次调用，足够每日同步。注册地址：
https://serpapi.com/manage-api-key

抓取成功且数据有变化时，Action 会提交 `config/publications.json` 并推送
`main`，随后的 Pages workflow 会自动重新构建并上线。

## 本地运行

```bash
node scripts/fetch-publications.mjs "<scholar-id>"
npm run test:publications
```

脚本输出确定性 JSON，内容不变时不会改写文件。
