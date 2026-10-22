<!-- Modified for the Lightcorf fork (2026); original licence retained. See docs/fork-license-audit.md. -->
# Lightcorf

這是由 [@tcholewa](https://github.com/tcholewa) 維護的獨立黑白攝影分支，基於 ArtCraft 團隊與貢獻者開發的
[LightCraft](https://github.com/storytold/lightcraft)。本專案不是 ArtCraft 官方發行版，也不代表其認可。

本分支包含 ORF 支援、原創 Tri-X 風格預設與顆粒效果，以及 macOS 的 Core Image / 原生 Rust RAW 處理器選擇。
在設定的效能頁面切換處理器後，需要重新啟動應用程式。新匯入的 macOS RAW 照片預設銳化值為 15。

首次發佈僅提供原始碼。建置方式、限制與支援入口請參閱 [英文說明](README.md)。
預設分支為 `lightcorf`；現有程式識別、資料庫路徑和開發歷史保留。

程式碼繼續採用 [MIT](LICENSE-MIT) 或 [Apache-2.0](LICENSE-APACHE) 雙授權。
Copyright (c) 2026 ArtCraft Team and the LightCraft contributors.
`crates/segment` 僅採用 Apache-2.0；字型和其他資源保留各自的授權，模型權重不包含在內。
請參閱 [NOTICE](NOTICE)、[資源署名](assets/ATTRIBUTION.md) 和 [授權審查](docs/fork-license-audit.md)。
受限制的 ArtCraft 標誌已從發佈版本中移除；原始版權與貢獻者記錄保持不變。
