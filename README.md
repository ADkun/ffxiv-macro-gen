# FF14模板宏生成器

## 介绍

这是一款能够根据模板文件生成宏并复制到系统剪贴板的小工具。

## 使用方法

1. 在`macros`文件夹中新建任意文本文件。
2. 编辑该文本文件，写宏模板。见下方示例。
3. 双击运行`ffxiv-momacro-gen.exe`
4. 输入数字选择宏模板文件

## 宏模板规则

1. 索引必须从1开始。
2. 索引必须连续。

## 宏模板说明

### 鼠标指向宏

```
/micon [1]
/merror off
/ac [1] <mo>
/ac [1]
```
参数列表：
1. 技能名称（如：出卡I）

例如输入 `出卡I` 那么最终会生成的宏为：
```
/micon 出卡I
/merror off
/ac 出卡I <mo>
/ac 出卡I
```

### 技能防卡宏

```
/micon [1]
/mlock
/ac [1]
/hotbar copy [2] [4] [2] [5]
/hotbar set [1] [4] [3]
/p 无敌《[1]》 <se.1>
/wait 1
/hotbar copy [2] [5] [2] [4]
```
参数列表：
1. 技能名称（如：行尸走肉）
2. 职业名称（拥有该技能的职业，如：暗黑骑士）
3. 该技能在技能栏中的位置（1-12的其一）
4. 该技能当前所在的热键栏编号（1-10的其一）
5. 要将该技能当前的热键栏复制到的热键栏编号（1-10的其一）（该热键栏应当是空的，且没有设置为共用热键栏）

例如输入 `行尸走肉 暗黑骑士 3 3 9`，那么输出是
```
/micon 行尸走肉
/mlock
/ac 行尸走肉
/hotbar copy 暗黑骑士 3 暗黑骑士 9
/hotbar set 行尸走肉 3 3
/p 无敌《行尸走肉》 <se.1>
/wait 1
/hotbar copy 暗黑骑士 9 暗黑骑士 3
```

## 如何贡献

您可以提交模板文件到`macros`文件夹中，并发起PR