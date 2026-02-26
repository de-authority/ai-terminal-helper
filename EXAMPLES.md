# Aish - AI Shell Helper 使用示例

本文档提供了更多实际使用示例和场景。

## 基础使用

### 1. 快速提问
```bash
# 直接提问
aish "什么是HTTP状态码404？"

# 使用更强大的模型
aish --model glm-4.7 "详细解释一下Rust的所有权机制"

# 使用本地或其他API
aish --api-base http://localhost:11434/v1 "你好"
```

### 2. 管道输入
```bash
# 分析错误日志
tail -n 100 /var/log/nginx/error.log | aish "分析这些错误日志，找出常见问题"

# 查看系统负载
uptime | aish "当前系统负载如何，需要优化吗？"

# 解析配置文件
cat /etc/nginx/nginx.conf | aish "这个Nginx配置做了什么？"

# 分析JSON数据
curl -s https://api.github.com/repos/rust-lang/rust | aish "总结这个仓库的信息"
```

### 3. 历史记录分析
```bash
# 查看最近的命令
aish --history "我最近在做什么项目？"

# 分析使用模式
aish --history "找出我使用频率最高的命令"

# 安全检查
aish --history "检查我的命令历史中是否有危险操作"
```

## 系统管理场景

### 监控和诊断
```bash
# 检查磁盘使用
df -h | aish "哪个分区空间最紧张？"

# 内存分析
free -h | aish "内存使用情况如何，是否需要释放？"

# 网络连接
ss -tulpn | aish "这些连接中哪些是可疑的？"

# 进程检查
ps aux | head -20 | aish "找出占用资源最多的进程"
```

### 日志分析
```bash
# 系统日志
journalctl -n 100 | aish "最近的系统日志有什么异常？"

# Docker日志
docker logs --tail 100 my-container | aish "容器有什么问题？"

# 应用日志
tail -f app.log | aish "实时监控日志并提醒异常"
```

## 开发场景

### 代码分析
```bash
# 分析代码结构
find src -name "*.rs" | xargs wc -l | aish "代码行数统计和结构分析"

# Git提交历史
git log --oneline -20 | aish "总结最近的开发进度"

# Git diff分析
git diff HEAD~1 | aish "这次修改做了什么？"
```

### 调试和问题解决
```bash
# 编译错误
cargo build 2>&1 | aish "帮助解决这些编译错误"

# 测试失败
cargo test 2>&1 | aish "为什么测试失败了？"

# 代码审查
git diff | aish "审查这些代码修改"
```

### 文档和注释
```bash
# 生成文档注释
cat src/main.rs | aish "为这个Rust文件生成文档注释"

# 代码解释
cat algorithm.rs | aish "解释这段算法的实现原理"

# 重构建议
cat legacy_code.py | aish "提供重构建议"
```

## 数据处理

### CSV和数据文件
```bash
# 分析CSV
head -50 data.csv | aish "这个CSV文件的列是什么意思？"

# 统计分析
cat sales.csv | aish "计算总销售额和平均订单金额"

# 数据格式转换
cat data.json | aish "将这个JSON转换为CSV格式"
```

### 日志和文本处理
```bash
# 提取信息
grep "ERROR" app.log | aish "提取并总结所有错误信息"

# 格式化输出
cat unformatted.txt | aish "格式化这段文本"

# 文本翻译
cat english.txt | aish "翻译成中文"
```

## 高级使用技巧

### 组合命令
```bash
# 查找大文件并分析
find . -type f -size +100M | aish "这些大文件是否可以删除？"

# 查找并解释进程
ps aux | grep python | aish "这些Python进程在做什么？"

# 分析网络请求
tcpdump -i any -c 10 | aish "分析这些网络包"
```

### 自动化脚本集成
```bash
#!/bin/bash
# check_health.sh - 系统健康检查脚本

echo "=== CPU Usage ==="
top -bn1 | head -20 | aish "CPU使用情况如何？"

echo "=== Disk Usage ==="
df -h | aish "磁盘空间是否足够？"

echo "=== Memory ==="
free -h | aish "内存使用是否正常？"

echo "=== Recent Errors ==="
journalctl -p err -n 10 | aish "最近的错误是什么？"
```

### 定时任务
```bash
# crontab 示例
# 每天早上9点检查系统健康
0 9 * * * /path/to/check_health.sh | aish "生成健康报告" | mail -s "每日系统报告" admin@example.com
```

## Shell集成示例

在使用 `source ai-integration.sh` 后：

```bash
# 快速提问
a "如何查找占用端口80的进程？"

# 上一个命令
ls -la | last "这个目录有什么问题？"

# 分析文件
analyze README.md "总结主要内容"

# 代码审查
review src/main.rs

# 解释命令
explain "sed -i 's/foo/bar/g' file.txt"
```

## 注意事项

1. **API限制**: 注意API的速率限制和成本
2. **敏感信息**: 避免将密码、密钥等敏感信息发送给AI
3. **输出长度**: 过长的输入可能会被截断
4. **网络依赖**: 需要稳定的网络连接
5. **语言选择**: 可以在问题中指定回复语言

## 故障排查

```bash
# 检查API密钥
echo $OPENAI_API_KEY

# 测试连接
aish --model glm-4.7 "测试连接"

# 查看详细错误
aish "test" 2>&1 | tee error.log

# 使用不同的API端点
aish --api-base https://api.anthropic.com/v1 "hello"
```

## 最佳实践

1. **明确的问题**: 尽量具体地描述问题
2. **上下文信息**: 提供足够的上下文
3. **分步解决**: 对于复杂问题，可以分步提问
4. **验证结果**: 不要完全依赖AI的回答，要验证
5. **保存有用答案**: 将有用的答案保存到文档中

## 创意用法

```bash
# 代码重构
cat old_code.py | aish "用现代Python风格重写这段代码"

# 学习新技术
curl -s https://api.github.com/repos/torvalds/linux/commits | aish "最近的Linux内核更新"

# 生成测试
cat module.py | aish "为这个模块生成单元测试"

# 文档生成
find . -name "*.rs" | xargs cat | aish "生成项目文档"
```
