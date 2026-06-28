#!/usr/bin/env bash

# 输入：（以下只要在参数里出现空格的，一律需要加单双引号！）
# --project_name <项目名字（以中短横线为分隔符，全小写）>
# --project_underline_name <项目名字（以下划线为分隔符，全小写）>
# --project_stylized_name <项目全称（填入自己项目的全称，随便填，可以有空格，可以大写，但是尽量不要有中文。。）>
# --project_version <项目版本号（以点为分隔符，由数字做分隔。一般是有 3 个字符。）>
# --project_author <项目作者（可以填入自己的英文名，以及自己的邮箱。）>
# --project_description <项目描述（可以填入项目的简要描述。尽量以英文写。）>
# --project_license <项目许可证（可以填入项目的许可证类型。建议填入“All Rights Reserved”）>
# --project_identifier <项目全局唯一标识（建议填入：“com.作者名.作品名”，其中作品名只能由 26 个英文字母并且全部小写组成！不能加短横线！）>

declare -A pattern_map=(
    ["--project_name"]="ren-rs-refactor"
    ["--project_underline_name"]="ren_rs_refactor"
    ["--project_stylized_name"]="Ren Rs Refactor"
    ["--project_version"]="0.1.0"
    ["--project_author"]="xphost <xphost@foxmail.com>"
    ["--project_description"]="VN Engine for Ren Rs!"
    ["--project_license"]="Apache 2.0"
    ["--project_identifier"]="com.xphost.renrsrefactor"
)
declare -A replacements
while [[ $# -gt 0 ]]; do
    key="$1"
    if [[ -n "${pattern_map[$key]}" ]]; then
        if [[ -z "$2" ]]; then
            echo "错误: 参数 $key 缺少对应的值"
            exit 1
        fi
        old="${pattern_map[$key]}"
        new="$2"
        replacements["$old"]="$new"
        shift 2
    else
        echo "未知参数: $key"
        echo "支持的参数: ${!pattern_map[@]}"
        exit 1
    fi
done
missing=()
for arg in "${!pattern_map[@]}"; do
    if [[ -z "${replacements[${pattern_map[$arg]}]}" ]]; then
        missing+=("$arg")
    fi
done
if [[ ${#missing[@]} -gt 0 ]]; then
    echo "错误: 缺少必需的参数: ${missing[@]}"
    exit 1
fi
if [[ "$OSTYPE" == "darwin"* ]]; then
    SED_INLINE="sed -i ''"
else
    SED_INLINE="sed -i"
fi
find . -type f ! -path "./change.sh" -exec grep -Il . {} \; | while IFS= read -r file; do
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
old_name_str="${pattern_map["--project_name"]}"
new_name_str="${replacements["$old_name_str"]}"
if [[ -n "$new_name_str" ]]; then
    find . -depth -name "*$old_name_str*" ! -path "./change.sh" | while IFS= read -r path; do
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
