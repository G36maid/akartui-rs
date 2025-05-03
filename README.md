# akartui-rs

A Rust-based TUI Akari (Light Up) game.

## App Layout & Design

### 整體結構

- **上方**：資訊欄（Puzzle ID、類型、作者、尺寸、來源、說明等）
- **中間**：左右分割
    - **左側**：Archive 題庫列表（1~750 題，可上下選擇，Enter 開始該題）
    - **右側**：根據狀態顯示內容
        - 主選單（New Game, Archive, Settings, Help, Exit）
        - 遊戲畫面（棋盤）
        - 設定
        - 說明
        - 離開確認
- **下方**：操作提示（根據狀態顯示快捷鍵說明）

---

### Menu（主選單）

- New Game：隨機開始一題
- Archive：切換到題庫選擇
- Settings：遊戲設定（預留）
- Help：顯示說明
- Exit：離開遊戲

---

### Archive（題庫選擇）

- 左側顯示 1~750 題目列表，可上下移動
- 右側可顯示該題 metadata 或預覽（預留）
- 按 Enter 開始該題

---

### Game（遊戲畫面）

- 上方顯示 puzzle 資訊
- 中間顯示棋盤（動態大小）
- 下方顯示操作提示
- 支援操作：
    - 移動游標（↑↓←→）
    - 放/移除燈泡（Space）
    - 放/移除旗子（P）
    - 返回主選單（Q）

---

### Settings（設定）

- 預留

---

### Help（說明）

- 顯示所有快捷鍵與操作說明

---

## 遊戲資料結構

- Puzzle ID: 1~750
- 棋盤大小：動態
- 格子類型：
    - Wall（牆）
    - Target（帶數字的牆，0~4）
    - Empty（空格）
- 狀態：
    - 牆
    - 燈光（被照亮）
    - 黑暗（未照亮）
- 玩家操作：
    - 放/移除燈泡
    - 放/移除旗子

---

## TODO

- [x] Archive 題庫選擇與啟動
- [x] 主選單與隨機遊戲
- [x] 棋盤顯示與操作
- [ ] Settings 畫面
- [ ] Archive 預覽/metadata 顯示
- [ ] 完善勝負判定、提示等

---

## 操作說明

- ↑↓←→：移動游標或選單
- Enter：選擇/開始
- Space：放/移除燈泡
- P：放/移除旗子
- Q：返回/離開
- G：隨機開始新遊戲（主選單）

---

## 其他

- 使用 [ratatui](https://github.com/ratatui-org/ratatui) 實現 TUI
- Puzzle 檔案存放於 `archive/` 目錄
