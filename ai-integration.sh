#!/bin/bash
# Aish - AI Shell Helper Shell Integration
# 将此文件添加到你的 .bashrc 或 .zshrc 中: source /path/to/ai-integration.sh

# 检查aish命令是否可用
if ! command -v aish &> /dev/null; then
    echo "Warning: 'aish' command not found. Install with: cargo install --path ai-terminal-helper"
    return
fi

# 快捷别名
alias ask='aish'

# 快速提问（无需引号）
a() {
    aish "$@"
}

# 分析上一个命令的输出
last() {
    local question="${1:-分析上一个命令的输出}"
    fc -ln -1 | aish "$question"
}

# 分析上一个命令的输出（完整输出，包含stderr）
last-full() {
    local question="${1:-分析上一个命令的输出}"
    local cmd=$(fc -ln -1)
    eval "$cmd" 2>&1 | aish "$question"
}

# 查看最近的命令历史
hist() {
    local question="${1:-分析我最近的命令历史}"
    aish --history "$question"
}

# 分析文本文件
analyze() {
    if [ -z "$1" ]; then
        echo "Usage: analyze <file> [question]"
        return 1
    fi
    local file="$1"
    local question="${2:-分析这个文件}"

    if [ ! -f "$file" ]; then
        echo "Error: File '$file' not found"
        return 1
    fi

    cat "$file" | aish "$question"
}

# 解释命令
explain() {
    if [ -z "$1" ]; then
        echo "Usage: explain <command>"
        return 1
    fi
    aish "解释命令: $1"
}

# 总结文件内容
summarize() {
    if [ -z "$1" ]; then
        echo "Usage: summarize <file>"
        return 1
    fi

    if [ ! -f "$1" ]; then
        echo "Error: File '$1' not found"
        return 1
    fi

    cat "$1" | aish "总结这个文件的内容"
}

# 代码审查
review() {
    if [ -z "$1" ]; then
        echo "Usage: review <file>"
        return 1
    fi

    if [ ! -f "$1" ]; then
        echo "Error: File '$1' not found"
        return 1
    fi

    cat "$1" | aish "审查这段代码，指出问题和改进建议"
}

# 查看帮助
aish-help() {
    cat << EOF
Aish - AI Shell Helper 集成命令:

基础命令:
  aish [question]            - 直接向AI提问
  a [question]              - 快捷提问（无引号）
  ask [question]            - ask的别名

历史和命令分析:
  last [question]           - 分析上一个命令
  last-full [question]      - 分析上一个命令的完整输出
  hist [question]           - 分析命令历史
  explain <command>         - 解释某个命令

文件分析:
  analyze <file> [question] - 分析文件内容
  summarize <file>          - 总结文件内容
  review <file>             - 代码审查

示例:
  a "如何在Linux中查找大文件？"
  ls -la | last "这个目录有什么问题？"
  hist "我最近在做什么？"
  analyze error.log "找出错误原因"
  explain "grep -r 'pattern' ."
  review src/main.rs

提示:
  - 确保设置了 OPENAI_API_KEY 环境变量（格式：id.secret）
  - 使用 --model 选项切换不同模型
  - 使用 --history 查看完整的shell历史

EOF
}
