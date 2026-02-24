# Phi-Save-Codec-Bind-Python

Phi-Save-Codec 的 Python 绑定，用于解析和构建 Phigros 的云端存档格式。

## 安装

```bash
pip install phi_save_codec
```

## 使用

```python
from phi_save_codec import PhiSaveCodec, PhiSaveCodecError

# 初始化编解码器
codec = PhiSaveCodec()

# 解析二进制数据
try:
    user_data = codec.parse_user(binary_data)
    print(user_data)
except PhiSaveCodecError as e:
    print(f"解析失败: {e}")

# 构建二进制数据
try:
    binary_data = codec.build_user(user_dict)
except PhiSaveCodecError as e:
    print(f"构建失败: {e}")
```

## 支持的操作

- `parse_user()` / `build_user()` - 用户数据
- `parse_summary()` / `build_summary()` - 摘要数据
- `parse_game_record()` / `build_game_record()` - 游戏记录
- `parse_game_progress()` / `build_game_progress()` - 游戏进度
- `parse_game_key()` / `build_game_key()` - 游戏密钥
- `parse_settings()` / `build_settings()` - 设置数据

## 异常处理

所有 API 方法在错误时抛出 `PhiSaveCodecError` 异常：

```python
try:
    data = codec.parse_user(binary_data)
except PhiSaveCodecError as e:
    # 获取详细的错误信息
    print(f"错误: {e}")
```

## 内存管理

该库自动处理与 WASM 模块之间的内存管理，包括：
- 分配和释放内存
- 获取和清空错误信息
- 确保在函数调用前后正确管理临时内存

## 依赖

- `msgpack` - 数据序列化
- `wasmtime` - WASM 运行时支持