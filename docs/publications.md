# Publications 自动同步

Publications 模块的数据来自 `config/publications.json`。该文件由
`scripts/fetch-publications.mjs` 生成，构建时会覆盖 `config/site.toml` 中
publications 的 `items`、`stats`、`total_value`、`citations_value` 和
`last_updated` 字段。

## 数据来源

脚本使用 Playwright 访问 Google Scholar 个人主页，抓取每篇论文的：

- 论文标题
- 合作者姓名（主页列表截断时会进入论文详情页补全）
- 发表年份
- 被引次数
- 期刊/会议名称与 Scholar 论文页链接

同时根据论文计算总数、总被引和近五年年度统计，并把抓取日期写入
`last_updated`，该时间会显示在模块末尾。

## 定时任务

`.github/workflows/publications.yml` 每天北京时间 05:00（UTC 21:00）运行，
也支持手动触发。工作流读取仓库变量 `SCHOLAR_ID`（或同名 secret）：

```bash
gh variable set SCHOLAR_ID --body "<google-scholar-user-id>"
```

抓取成功且数据有变化时，Action 会提交 `config/publications.json` 并推送
`main`，随后的 Pages workflow 会自动重新构建并上线。

## 本地运行

```bash
node scripts/fetch-publications.mjs "<scholar-id>"
```

脚本输出确定性 JSON，内容不变时不会改写文件。
