import msgpack
from wasmtime import Store, Module, Instance, Engine, Memory

class PhiSaveCodec:
    def __init__(self, wasm_path: str = "phi_save_codec.wasm"):
        self._engine = Engine()
        self._store = Store(self._engine)
        self._module = Module.from_file(self._engine, wasm_path)
        self._instance = Instance(self._store, self._module, [])
        self._exports = self._instance.exports(self._store)
        self._mem:Memory = self._exports["memory"]
    
    def _free(self, ptr: int, size: int):
        if ptr == 0 or size == 0:
            raise ValueError("无效指针或大小")
        self._exports["free"](self._store, ptr, size)
        
    def _malloc(self, size: int) -> int:
        if size == 0:
            raise ValueError("无效大小")
        return self._exports["malloc"](self._store, size)

    def _invoke(self, func_name: str, in_data: bytes) -> bytes:
        # 写入数据
        in_size = len(in_data)
        in_ptr = self._malloc(in_size)
        self._mem.write(self._store,in_data,in_ptr)
        # 调用函数
        out_size, out_ptr = self._exports[func_name](self._store, in_ptr, in_size)
        # 读取数据
        out_data = self._mem.read(self._store,out_ptr,out_ptr+out_size)
        # 释放内存
        self._free(in_ptr,in_size)
        self._free(out_ptr,out_size)
        return out_data

    def _parse(self, name: str, data: bytes) -> dict:
        out = self._invoke(f"parse_{name}", data)
        return msgpack.unpackb(out)

    def _build(self, name: str, obj: dict) -> bytes:
        return self._invoke(f"build_{name}", msgpack.packb(obj))

    def memory_size(self) -> int:
        return self._mem.data_len(self._store)
    
    def parse_user(self, data: bytes) -> dict: return self._parse("user", data)
    def build_user(self, obj: dict) -> bytes: return self._build("user", obj)

    def parse_summary(self, data: bytes) -> dict: return self._parse("summary", data)
    def build_summary(self, obj: dict) -> bytes: return self._build("summary", obj)

    def parse_game_record(self, data: bytes) -> dict: return self._parse("game_record", data)
    def build_game_record(self, obj: dict) -> bytes: return self._build("game_record", obj)

    def parse_game_progress(self, data: bytes) -> dict: return self._parse("game_progress", data)
    def build_game_progress(self, obj: dict) -> bytes: return self._build("game_progress", obj)

    def parse_game_key(self, data: bytes) -> dict: return self._parse("game_key", data)
    def build_game_key(self, obj: dict) -> bytes: return self._build("game_key", obj)

    def parse_settings(self, data: bytes) -> dict: return self._parse("settings", data)
    def build_settings(self, obj: dict) -> bytes: return self._build("settings", obj)