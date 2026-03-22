#!/bin/bash
# 推送到 GitHub 脚本

if [ -z "$1" ]; then
    echo "用法: $0 <GitHub用户名>"
    echo "示例: $0 johndoe"
    exit 1
fi

USERNAME=$1
cd /home/mey/woof

echo "=== 配置远程仓库 ==="
git remote remove origin 2>/dev/null
git remote add origin "https://github.com/$USERNAME/woof.git"

echo "=== 远程仓库 ==="
git remote -v

echo ""
echo "=== 推送到 GitHub ==="
echo "如果提示输入密码，请使用 GitHub Personal Access Token"
echo "创建 Token: https://github.com/settings/tokens"
echo ""
git push -u origin master

echo ""
echo "=== 完成 ==="
echo "仓库地址: https://github.com/$USERNAME/woof"
