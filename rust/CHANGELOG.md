# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog],
and this project adheres to [Semantic Versioning].

## [Unreleased]


### TODO

- [ ] 指针类型处理
- [ ] 枚举解析
- [ ] 函数定义

### Added

- 

### Changed

### Removed

### Fixed

## [0.0.2] - 2022-08-27

### Added

- 简化的全局变量声明解析,支持char和int两种类型.
- 简单的表达式树定义,遍历,求值
- 参照已有算法,实现了两个表达式树的pretty print算法; 其中中缀版本一开始直接翻译的C++代码, 实现
  得比较艰难.
- 解析声明的代码结构调整,初步实现递归下降语法解析的结构
### Fixed

- peek迭代器的使用bug

## [0.0.1] - 2022-08-14

- initial release
- 开始使用rust重新开发,完成了简易的词法解析功能

<!-- Links -->
[keep a changelog]: https://keepachangelog.com/en/1.0.0/
[semantic versioning]: https://semver.org/spec/v2.0.0.html

<!-- Versions -->
[unreleased]: https://github.com/davidfeng/ci/compare/v0.0.3...HEAD
[0.0.2]: https://github.com/davidfeng/ci/releases/tag/v0.0.2
[0.0.1]: https://github.com/davidfeng/ci/releases/tag/v0.0.1