# Aish - AI Shell Helper 项目结构

```
ai-terminal-helper/
├── Cargo.toml              # 项目配置和依赖
├── Cargo.lock              # 依赖锁定文件
├── README.md               # 主要文档
├── EXAMPLES.md             # 使用示例
├── .env.example            # 环境变量示例
├── .gitignore              # Git忽略规则
├── Makefile                # 构建和安装脚本
├── ai-integration.sh       # Shell集成脚本
└── src/
    └── main.rs             # 主程序代码
```

## 项目说明

这是一个Rust编写的命令行工具，用于与AI模型交互。主要功能包括：

1. **直接提问**: 命令行中直接向AI提问
2. **管道分析**: 通过管道将命令输出发送给AI分析
3. **历史记录**: 读取和分析shell命令历史
4. **灵活配置**: 支持自定义API端点和模型

## 核心功能实现

### 1. 命令行参数解析
使用 `clap` 库实现强大的命令行参数解析：
- `--api-key`: API密钥
- `--api-base`: API基础URL
- `--model`: 模型名称
- `--history`: 启用历史记录模式
- `[question]`: 可选的问题

### 2. AI交互
使用 `reqwest` 库实现HTTP客户端，支持：
- 智谱AI API格式
- 自定义API端点
- 异步请求处理
- 错误处理

### 3. 历史记录读取
自动检测并读取多种shell的历史文件：
- Bash: `~/.bash_history`
- Zsh: `~/.zsh_history`
- Fish: `~/.local/share/fish/fish_history`

### 4. 管道输入处理
使用 `atty` 库检测stdin，支持：
- 管道输入分析
- 交互式输入
- 自动检测输入来源

## 依赖说明

```toml
tokio = "1.35"          # 异步运行时
reqwest = "0.11"        # HTTP客户端
clap = "4.5"            # 命令行参数解析
serde = "1.0"           # 序列化/反序列化
serde_json = "1.0"      # JSON处理
atty = "0.2"            # 终端检测
dirs = "5.0"            # 目录路径
```

## 快速开始

1. **安装**:
    ```bash
    cd ai-terminal-helper
    make install
    ```

2. **配置**:
    ```bash
    export OPENAI_API_KEY="your-api-key"
    ```

3. **使用**:
    ```bash
    aish "你好"
    ls -la | aish "分析这个输出"
    aish --history "分析我的命令历史"
    ```

## Shell集成

为了更方便的使用，可以加载shell集成脚本：

```bash
source ai-terminal-helper/ai-integration.sh
```

集成后可用的快捷命令：
- `a [question]`: 快速提问
- `last [question]`: 分析上一个命令
- `hist [question]`: 分析命令历史
- `analyze <file> [question]`: 分析文件
- `review <file>`: 代码审查

## 开发说明

### 构建
```bash
make build      # 构建发布版本
make dev        # 构建调试版本
make run        # 运行程序
```

### 测试
```bash
make test       # 运行测试
```

### 清理
```bash
make clean      # 清理构建产物
```

## 扩展建议

1. **更多AI模型支持**: 添加对更多API提供商的支持
2. **会话模式**: 实现多轮对话功能
3. **历史保存**: 保存对话历史到本地
4. **配置文件**: 支持从配置文件读取设置
5. **流式输出**: 实现流式响应显示
6. **模板系统**: 预定义常用提示模板
7. **代码补全**: 集成shell自动补全
8. **本地缓存**: 缓存常见问题的答案

## 安全注意事项

1. 不要在命令行参数中直接传递API密钥（会被记录在历史中）
2. 使用环境变量存储敏感信息
3. 避免将机密信息发送给AI
4. 定期轮换API密钥
5. 检查输入内容，防止注入攻击

## 许可证

MIT License
