#!/usr/bin/env bash
set -euo pipefail

CONFIG_FILE="ren-rs.config.json"

if [[ ! -f "$CONFIG_FILE" ]]; then
  echo "错误: 找不到配置文件 $CONFIG_FILE"
  exit 1
fi

if ! command -v jq &> /dev/null; then
  echo "错误: 未安装 'jq' 命令，请先安装 jq 来解析 JSON 文件。"
  exit 1
fi

OLD_PROJECT_NAME="ren-rs-refactor"
OLD_PROJECT_UNDERLINE_NAME="ren_rs_refactor"
OLD_PROJECT_STYLIZED_NAME="Ren Rs Refactor"
OLD_PROJECT_IDENTIFIER="com.xphost.renrsrefactor"
OLD_PROJECT_VERSION="0.1.0"
OLD_PROJECT_AUTHOR="xphost <xphost@foxmail.com>"
OLD_PROJECT_DESCRIPTION="VN Engine for Ren Rs!"
OLD_PROJECT_LICENSE="Apache 2.0"

NEW_PROJECT_NAME=$(jq -r '.package_name' "$CONFIG_FILE")
NEW_PROJECT_UNDERLINE_NAME=$(jq -r '.package_underline_name' "$CONFIG_FILE")
NEW_PROJECT_STYLIZED_NAME=$(jq -r '.project_stylized_name' "$CONFIG_FILE")
NEW_PROJECT_IDENTIFIER=$(jq -r '.package_identifier' "$CONFIG_FILE")
NEW_PROJECT_VERSION=$(jq -r '.package_version' "$CONFIG_FILE")
NEW_PROJECT_AUTHOR=$(jq -r '.project_author' "$CONFIG_FILE")
NEW_PROJECT_DESCRIPTION=$(jq -r '.project_description' "$CONFIG_FILE")
NEW_PROJECT_LICENSE=$(jq -r '.project_license' "$CONFIG_FILE")

# 将旧值映射到新值
declare -A replacements=(
  ["$OLD_PROJECT_NAME"]="$NEW_PROJECT_NAME"
  ["$OLD_PROJECT_UNDERLINE_NAME"]="$NEW_PROJECT_UNDERLINE_NAME"
  ["$OLD_PROJECT_STYLIZED_NAME"]="$NEW_PROJECT_STYLIZED_NAME"
  ["$OLD_PROJECT_IDENTIFIER"]="$NEW_PROJECT_IDENTIFIER"
  ["$OLD_PROJECT_VERSION"]="$NEW_PROJECT_VERSION"
  ["$OLD_PROJECT_AUTHOR"]="$NEW_PROJECT_AUTHOR"
  ["$OLD_PROJECT_DESCRIPTION"]="$NEW_PROJECT_DESCRIPTION"
  ["$OLD_PROJECT_LICENSE"]="$NEW_PROJECT_LICENSE"
)

# 检查是否所有字段都有值
missing=()
for old in "${!replacements[@]}"; do
  if [[ -z "${replacements[$old]}" ]]; then
    missing+=("$old")
  fi
done

if [[ ${#missing[@]} -gt 0 ]]; then
  echo "错误: JSON 配置文件中存在空字段或提取失败！"
  exit 1
fi

# 根据系统类型设置 sed 命令
if [[ "$OSTYPE" == "darwin"* ]]; then
  SED_INLINE="sed -i ''"
else
  SED_INLINE="sed -i"
fi

# 排除当前脚本自身和 JSON 配置文件，防止配置文件中的旧值也被替换
EXCLUDE_PATHS="! -path \"./change.sh\" ! -path \"./$CONFIG_FILE\" ! -path \"./change_by_json.sh\""

# 替换文件内容
find . -type f $EXCLUDE_PATHS -exec grep -Il . {} \; | while IFS= read -r file; do
  need_replace=false
  for old in "${!replacements[@]}"; do
    if grep -Fq "$old" "$file"; then
      need_replace=true
      break
    fi
  done

  if $need_replace; then
    for old in "${!replacements[@]}"; do
      new="${replacements[$old]}"
      eval "$SED_INLINE 's#$old#$new#g' \"$file\""
    done
    echo "$file"
  fi
done

# 替换文件名
old_name_str="$OLD_PROJECT_NAME"
new_name_str="${replacements["$old_name_str"]}"

if [[ -n "$new_name_str" ]]; then
  find . -depth -name "*$old_name_str*" $EXCLUDE_PATHS | while IFS= read -r path; do
    dir=$(dirname "$path")
    base=$(basename "$path")
    new_base="${base//$old_name_str/$new_name_str}"
    if [[ "$base" != "$new_base" ]]; then
      new_path="$dir/$new_base"
      mv "$path" "$new_path"
      echo "./$path -> ./$new_path"
    fi
  done
fi

echo "替换完成！"
