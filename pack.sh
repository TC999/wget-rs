#!/bin/bash

# 进入 bin 目录
cd bin

# 遍历所有子目录
for dir in */; do
  # 去掉最后的斜杠，获取目录名
  dirname=${dir%/}
  # 查找该目录下以 wget-rs 开头的文件（包含可执行文件）
  filepath="$dir"wget-rs*
  # 判断文件是否存在
  if ls $filepath 1> /dev/null 2>&1; then
    # 在 bin 上级目录下打包，压缩文件名为父级目录名
    tar czf "../${dirname}.tar.gz" $filepath
    echo "已打包: ${dirname}.tar.gz"
  fi
done