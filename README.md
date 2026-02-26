# Aish - AI Shell Helper

一个命令行辅助工具，让你可以方便地将命令输出直接发送给大模型进行分析，或者直接提问，还可以感知终端历史记录
此工具由vibe coding生成,当然, 由于总共代码长度不到200行,且功能相对单一, 因此本人并没有认真review. 此项目只有2个目的
- 满足本人的基本日常需求,命令行时,不想再粘贴输入到大模型的对话窗, 当然,已经有更成熟的开源产品.
- 测试具体的大模型能力,当前用glm 4.7, 生成这样的小工具没有多大挑战.

## 遇到了哪些问题
- 不同的大模型api套餐有明显区别,这在刚接触的时侯会有点弄不清状况,例如在使用glm coding plan时与普通的API调用端点不同,但大模型调用的报错可能没有那么直白,会提示模型不存在/额度不够,但其实是端点不对.
- Opencode 复制/粘贴功能影响效率.
以上2点,问大模型也很难得到准角的答案, 我也没有发现glm提供了方便的搜索方式,例如RAG增强,让用户可以直接从glm官方找到准确答案.即便在大模型非常好用的今天,世界仍然并不完美.

## 功能

- **直接提问**: 在命令行中直接向AI提问
- **管道分析**: 通过管道将命令输出发送给AI分析
- **历史记录**: 查看和分析你的shell命令历史
- **灵活配置**: 支持自定义API端点和模型

## 安装

```bash
cd ai-terminal-helper
cargo install --path .
```

## 配置

设置API密钥（选择一种方式）：

```bash
# 方式1: 环境变量
export OPENAI_API_KEY="your-api-key"

# 方式2: 使用 --api-key 参数
aish --api-key "your-api-key" "你的问题"
```

## 使用方法

### 1. 直接提问

```bash
aish "如何在Linux中查找大文件？"
aish "解释一下什么是闭包"
```

### 2. 分析命令输出

```bash
# 查看并分析错误日志
cat error.log | aish "分析这个错误日志"
ls -la | aish "解释这个目录的结构"
ps aux | aish "找出占用CPU最高的进程"
```

### 3. 查看历史记录

```bash
# 查看最近的命令历史
aish --history

# 分析历史记录
aish --history "根据我的命令历史，我最近在做什么？"
```

### 4. 组合使用

```bash
# 查看文件并用AI分析
docker logs container_id | aish "这些日志有什么问题？"

# 查看系统资源
df -h | aish "分析磁盘使用情况"
```

## 命令行选项

```
USAGE:
    aish [OPTIONS] [QUESTION]

OPTIONS:
      --api-key <API_KEY>      API密钥
      --api-base <API_BASE>    API基础URL [默认: https://api.z.ai/api/coding/paas/v4]
  -m, --model <MODEL>          模型名称 [默认: glm-4.7]
  -H, --history                显示shell历史记录
  -h, --help                   显示帮助信息
```

## 使用的示例

### 系统管理
```bash
# 分析网络连接
netstat -tulpn | aish "这些连接中哪些是可疑的？"

# 检查进程状态
systemctl status nginx | aish "nginx状态如何，有问题吗？"
```

### 开发调试
```bash
# 分析编译错误
cargo build 2>&1 | aish "帮我解决这些编译错误"

# 查看Git日志
git log --oneline -10 | aish "总结最近的提交"
```

### 数据处理
```bash
# 分析CSV数据
head -100 data.csv | aish "这个CSV文件的结构是什么？"

# JSON格式化
cat data.json | aish "格式化并解释这个JSON"
```

## 支持的Shell

工具会自动检测以下shell的历史文件：
- Bash: `~/.bash_history`
- Zsh: `~/.zsh_history` 或 `~/.zhistory`
- Fish: `~/.local/share/fish/fish_history`

## 提示

1. **设置别名**: 为方便使用，可以在shell配置中添加别名
   ```bash
   alias aish='~/.cargo/bin/aish'
   ```

2. **快速提问**: 可以创建一个简短的别名
   ```bash
   alias ask='aish'
   ```

3. **默认模型**: 可以创建一个shell函数来使用特定的模型
   ```bash
   aishigh() {
       aish --model glm-4.7 "$@"
   }
   ```

## 注意事项

- 确保已设置API密钥（格式：id.secret）
- 默认使用智谱AI的Coding Plan端点
- 输入内容不要太长，有些API有长度限制
- 使用管道时，AI会分析所有输出内容
- 历史记录功能需要shell历史文件存在且可读

## 许可证

MIT
