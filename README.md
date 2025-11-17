# Plants vs Zombies - Rust Rewrite

一个使用 Rust 和 Bevy 游戏引擎重写的植物大战僵尸游戏。

## 项目说明

本项目基于原版植物大战僵尸源码 (https://github.com/TC999/re-plants-vs-zombies) 进行 Rust 重写，采用 Bevy 0.11 游戏引擎构建。

## 功能特性

### 当前已实现

- ✅ 基础网格系统（9x5 草坪）
- ✅ 5种植物类型
  - 豌豆射手 (Peashooter) - 100阳光
  - 向日葵 (Sunflower) - 50阳光
  - 坚果墙 (WallNut) - 50阳光
  - 寒冰射手 (SnowPea) - 175阳光
  - 双发射手 (Repeater) - 200阳光

- ✅ 5种僵尸类型
  - 普通僵尸 (Normal Zombie)
  - 路障僵尸 (ConeHead Zombie)
  - 铁桶僵尸 (BucketHead Zombie)
  - 旗帜僵尸 (Flag Zombie)
  - 读报僵尸 (Newspaper Zombie)

- ✅ 核心游戏机制
  - 植物放置系统
  - 僵尸自动生成（每10秒）
  - 豌豆射击系统
  - 碰撞检测
  - 阳光生成和收集
  - 战斗系统（植物vs僵尸）

### 待实现功能

- ⬜ 更多植物类型（樱桃炸弹、土豆雷、大嘴花等）
- ⬜ 更多僵尸类型
- ⬜ 关卡系统
- ⬜ 游戏结束判定
- ⬜ 音效和背景音乐
- ⬜ 图形资源（精灵图）
- ⬜ 更好的UI界面
- ⬜ 保存/加载功能

## 构建和运行

### 前置要求

- Rust 1.70+ (推荐使用 rustup 安装)
- Linux系统需要安装以下依赖：
  ```bash
  # Ubuntu/Debian
  sudo apt-get install libx11-dev
  ```

### 构建项目

```bash
cargo build --release
```

### 运行游戏

```bash
cargo run --release
```

## 游戏控制

### 键盘控制
- **1** - 选择豌豆射手 (100阳光)
- **2** - 选择向日葵 (50阳光)
- **3** - 选择坚果墙 (50阳光)
- **4** - 选择寒冰射手 (175阳光)
- **5** - 选择双发射手 (200阳光)

### 鼠标控制
- **左键点击草坪** - 放置选中的植物
- **左键点击阳光** - 收集阳光

## 游戏规则

1. 游戏开始时你拥有 150 点阳光
2. 种植向日葵以持续产生阳光（每24秒产生25阳光）
3. 使用阳光种植攻击型植物来防御僵尸
4. 僵尸会从右侧不断出现并向左移动
5. 当僵尸遇到植物时会停下来攻击
6. 使用不同类型的植物制定防御策略

## 技术架构

### 使用的技术栈

- **Rust** - 系统编程语言，保证性能和安全性
- **Bevy 0.11** - 现代化的Rust游戏引擎，采用ECS架构
- **rand** - 随机数生成库

### 项目结构

```
plantsvszombies/
├── src/
│   ├── main.rs              # 主程序入口
│   ├── components/          # ECS组件
│   │   ├── mod.rs
│   │   ├── plant.rs         # 植物组件
│   │   ├── zombie.rs        # 僵尸组件
│   │   ├── projectile.rs    # 子弹组件
│   │   ├── sun.rs           # 阳光组件
│   │   └── position.rs      # 位置组件
│   └── systems/             # 游戏系统
│       ├── mod.rs
│       ├── input.rs         # 输入处理
│       ├── game.rs          # 游戏逻辑
│       ├── movement.rs      # 移动系统
│       ├── shooting.rs      # 射击系统
│       ├── projectile.rs    # 子弹移动
│       ├── collision.rs     # 碰撞检测
│       ├── sun.rs           # 阳光系统
│       └── spawning.rs      # 僵尸生成
├── Cargo.toml               # 项目依赖配置
└── README.md
```

### ECS架构说明

本项目采用 Entity-Component-System (ECS) 架构：

- **Entity（实体）**: 游戏中的对象（植物、僵尸、子弹等）
- **Component（组件）**: 实体的数据（健康值、攻击力、位置等）
- **System（系统）**: 处理具有特定组件的实体的逻辑

## 贡献指南

欢迎贡献代码！请：

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件

## 致谢

- 原版植物大战僵尸游戏及其开发团队 PopCap Games
- 原版源码项目: https://github.com/TC999/re-plants-vs-zombies
- Bevy 游戏引擎社区

## 开发计划

### 短期目标
- [ ] 添加游戏结束判定（僵尸到达左边界）
- [ ] 添加胜利条件（存活特定时间/击败特定数量僵尸）
- [ ] 改进UI显示
- [ ] 添加更多植物类型

### 长期目标
- [ ] 添加多个关卡
- [ ] 添加昼夜模式
- [ ] 添加迷你游戏模式
- [ ] 支持自定义关卡编辑器
- [ ] 多人对战模式
