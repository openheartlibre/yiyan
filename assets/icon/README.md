# 图标

- `make-icons.py`：下面四个文件的生成脚本（仓库根目录跑 `python3 assets/icon/make-icons.py`）。
  图标就是一个 Y：两条臂加一根竖干，形状靠脚本顶部的几个常数调。机器上没有 rsvg-convert，PDF 由脚本直接写最小文件。
- `yiyan-mark.svg`：README 页头的标记，深竹青底 + 米白的 Y。
- `logo.png`（1024×1024，带透明通道）：应用图标源文件。`apps/macos/scripts/bundle.sh` 打包时用 `sips` + `iconutil`
  生成 `Yiyan.icns`，生成物不进仓库。
- `menu.svg` / `menu.pdf`：macOS 输入法图标（22×16pt 的黑色模板图，系统只取 alpha，按深浅色反色）。
  打包时 `menu.pdf` 拷成 `yiyan-menu.pdf`，Info.plist 的图标键都指向它。为什么是这个形式和尺寸见
  `docs/design/architecture.md`「Info.plist 约定」。
- `windows/mode-zh.svg` / `mode-en.svg` / `mode-caps.svg`：Windows 任务栏的中 / 英 / A 图标源文件（16×16 画布，单色）。
  `windows/render-mode-icons.sh` 用 rsvg-convert + magick 栅格化成 16 / 20 / 24 / 32 四档的 8 位 alpha 蒙版，
  写到 `apps/windows/tsf/resources/mode/`，DLL 用 `include_bytes!` 嵌入、运行时按任务栏深浅色填色（`com/mode/icon.rs`）。
  改了 svg 重跑脚本，生成物随仓库提交。
