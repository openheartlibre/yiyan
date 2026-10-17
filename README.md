<p align="center">
  <img src="assets/icon/yiyan-mark.svg" alt="易言竹简图标" height="108">
</p>

<h1 align="center">易言 Yiyan</h1>

<p align="center"><strong>好好打字，顺手多记一个词。</strong></p>

<p align="center">
  <img src="https://img.shields.io/badge/status-beta-orange" alt="测试阶段">
  <a href="https://github.com/openheartlibre/yiyan/stargazers"><img src="https://img.shields.io/github/stars/openheartlibre/yiyan?style=flat&amp;label=Stars" alt="GitHub 星标"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT 协议"></a>
  <a href="docs/user/getting-started/install.md"><img src="https://img.shields.io/badge/macOS-13%2B-blue" alt="macOS 13 及以上"></a>
  <a href="docs/user/getting-started/install.md"><img src="https://img.shields.io/badge/Windows-10%2F11-blue" alt="Windows 10/11"></a>
  <a href="docs/user/getting-started/linux.md"><img src="https://img.shields.io/badge/Linux-Fcitx5%20manual-lightgrey" alt="Linux Fcitx5，需手动启用"></a>
</p>

> 由独立开发者自主设计并实现的开源输入法，采用 MIT 协议发布。

易言是一款输入法。日常打字流程保持不变：敲拼音、挑候选、写整句；候选旁边会附上一条译词，让语言积累悄悄融进每一次输入。译词只作参考，不会喧宾夺主遮住你要打的字。

## 获取与上手

- **macOS、Windows**：安装流程参见[使用文档](docs/user/getting-started/install.md)。
- **Linux**：Fcitx5 版本已可用，沿用系统自带候选面板；后台服务目前需手动拉起，具体步骤见 [Linux 安装说明](docs/user/getting-started/linux.md)。

macOS 与 Windows 版目前属于测试阶段。装好后先把易言设为当前输入法，再到「偏好设置 / 设置 → 通用」里挑一门想学的语言，即可开始打字。初次使用建议从[第一次输入](docs/user/getting-started/first-input.md)读起。

## 打字时的界面

```text
1  你好         hello
2  世界         world
3  谢谢         thanks
```

候选旁同一时间只展示一门学习语言的译词。当前可选英语、日语、西班牙语，也能把译词关掉。除此之外，易言还提供整句输入、简拼、拼写纠错、双拼、五笔等打法；本地整句模型会在你停顿时重新排序句候选。

「统计」页会汇总今日、近 7 天与累计的输入量，并记录学习语言的词汇情况。更多细节见[输入功能](docs/user/input/)、[译词与生词](docs/user/learning/translation.md)、[本地统计](docs/user/learning/statistics.md)、[按键与快捷键](docs/user/getting-started/keys.md)。

## 数据与隐私

拼音转换、词库检索、本地模型与输入习惯学习全部在你的设备上运行。输入量和词汇统计同样只留在本机，不会外传；使用易言无需注册账号。输入日志与统计数据分开存放，日志可在设置里关闭或清空，且不影响统计。检查更新时会向官网拉取版本列表，该功能可在设置中关闭。

可选的云联想默认处于关闭状态。一旦开启，易言会把当前输入及邻近文字直接发给你自己配置的 AI 服务商，用来换取候选或整句补全；这些请求不经过易言的服务器。具体发送范围与本机留存的数据，见[数据与日志](docs/user/help/data-and-logs.md)。

## 文档、反馈与参与开发

- [使用文档](docs/user/)：涵盖安装、设置、输入、卸载与常见问题。
- [反馈问题或建议](https://github.com/openheartlibre/yiyan/issues/new/choose)；也可加入 [QQ 内测交流群](https://qm.qq.com/q/jBvn2gGTxm)。
- 想一起开发？从[开发文档](docs/)和[开发约定](docs/contributing.md)入手。

易言免费提供。代码以 [MIT](LICENSE) 协议开放；项目名称与 logo 不在代码授权范围内。随包附带的数据各有来源与许可，详见[数据来源清单](docs/design/landscape.md)。
