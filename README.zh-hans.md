<!-- Modified for the Lightcorf fork (2026); original licence retained. See docs/fork-license-audit.md. -->
# Lightcorf

这是由 [@tcholewa](https://github.com/tcholewa) 维护的独立黑白摄影分支，基于 ArtCraft 团队与贡献者开发的
[LightCraft](https://github.com/storytold/lightcraft)。本项目不是 ArtCraft 官方发行版，也不代表其认可。

本分支包含 ORF 支持、原创 Tri-X 风格预设与颗粒效果，以及 macOS 的 Core Image / 原生 Rust RAW 处理器选择。
在设置的性能页面切换处理器后，需要重启应用。新导入的 macOS RAW 照片默认锐化值为 15。

首次发布仅提供源代码。构建方法、限制和支持入口请参阅 [英文说明](README.md)。
默认分支为 `lightcorf`；现有程序标识、资料库路径和开发历史保留。

代码继续采用 [MIT](LICENSE-MIT) 或 [Apache-2.0](LICENSE-APACHE) 双许可证。
Copyright (c) 2026 ArtCraft Team and the LightCraft contributors.
`crates/segment` 仅采用 Apache-2.0；字体和其他资源保留各自的许可证，模型权重不包含在内。
请参阅 [NOTICE](NOTICE)、[资源署名](assets/ATTRIBUTION.md) 和 [许可证审查](docs/fork-license-audit.md)。
受限制的 ArtCraft 标志已从发布版本中移除；原始版权和贡献者记录保持不变。
