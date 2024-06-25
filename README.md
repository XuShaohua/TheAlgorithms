
# 数据结构与算法 - Rust 语言实现

使用 Rust 语言实现所有的数据结构与算法.

[在线浏览](https://algs.biofan.org), 或者[下载 pdf 文件](https://share.biofan.org/algs.pdf).

本文档包括了以下几个部分的内容:
1. 第一部分: 数据结构
2. 第二部分: 算法及其实现
3. 第三部分: 专题
4. 第四部分: leetcode 题解

## 反馈问题

欢迎 [反馈问题](https://github.com/xushaohua/TheAlgorithms/issues), 或者提交 PR.

## 搭建本地环境

想在本地搭建本文档的环境也是很容易的.

这些文档记录以 markdown 文件为主, 用 [mdbook](https://github.com/rust-lang/mdBook) 生成网页.

用cargo来安装它: `cargo install mdbook mdbook-linkcheck`

运行 `mdbook build` 命令, 会在`book/`目录里生成完整的电子书的网页版本.

在编写文档的同时, mdbook 工具可以检查文件变更, 按需自动更新.
使用 `mdbook serve` 命令启动一个本地的 web 服务器, 在浏览器中打开 [http://localhost:3000](http://localhost:3000).

### 生成 PDF

如果想生成 pdf, 需要安装 [mdbook-pandoc](https://github.com/max-heller/mdbook-pandoc),
用命令: `cargo install mdbook-pandoc`

并且安装 latex 相应的包:

```bash
sudo apt install pandoc librsvg2-bin texlive-latex-recommended latex-cjk-all texlive-xetex
```

安装好依赖之后, 运行 `./tools/generate-pdf.sh` 命令, 就会生成 `book-pandoc/intro-to-rust.pdf`.
