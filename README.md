# 数据结构与算法 - Rust 语言实现

使用 Rust 语言实现所有的数据结构与算法.

[在线浏览](https://algs.biofan.org)

本文档包括了以下几个部分的内容:

1. 第一部分: 数据结构
2. 第二部分: 算法及其实现
3. 第三部分: 专题
4. 第四部分: leetcode 题解

## 反馈问题

欢迎 [反馈问题](https://github.com/xushaohua/TheAlgorithms/issues), 或者提交 PR.

## 搭建本地环境

想在本地搭建本文档的环境也是很容易的, 这些文档记录以 markdown 文件为主,
用 [mdbook](https://github.com/rust-lang/mdBook) 生成网页:

1. 用cargo来安装它: `cargo install mdbook mdbook-linkcheck`
2. 运行 `mdbook build` 命令, 会在`book/`目录里生成完整的电子书的网页版本.
3. 使用 `mdbook serve` 命令监控文件变更, 并启动一个本地的 web 服务器,
   在浏览器中打开 [http://localhost:3000](http://localhost:3000)

### 生成 PDF

如果想生成 pdf, 需要安装 [mdbook-pandoc](https://github.com/max-heller/mdbook-pandoc):

- 运行 `./tools/install-pdf-deps.sh` 脚本安装相应的依赖包
- 运行 `./tools/generate-pdf.sh` 脚本, 就会生成 `book-pandoc/pdf/TheAlgorithms.pdf`

## 版权

本文档中附带的源代码文件依照 [GPL 3.0](LICENSE) 协议发布.
