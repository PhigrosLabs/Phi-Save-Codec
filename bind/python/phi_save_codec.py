import msgpack
from wasmtime import Store, Module, Instance, Engine, Memory

class PhiSaveCodecError(Exception):
    pass

class PhiSaveCodec:
    def __init__(self, wasm_path: str = "phi_save_codec.wasm"):
        self._engine = Engine()
        self._store = Store(self._engine)
        self._module = Module.from_file(self._engine, wasm_path)
        self._instance = Instance(self._store, self._module, [])
        self._exports = self._instance.exports(self._store)
        self._mem: Memory = self._exports["memory"] # pyright: ignore[reportAttributeAccessIssue]
    
    def _get_last_error(self) -> str:
        err_size, err_ptr = self._exports["psc_get_last_error"](self._store) # pyright: ignore[reportCallIssue]
        if err_ptr == 0 or err_size == 0:
            return ""
        try:
            error_bytes = self._mem.read(self._store, err_ptr, err_ptr + err_size)
            error_msg = error_bytes.decode('utf-8')
            self._free(err_ptr,err_size)
            return error_msg
        except Exception as e:
            return f"读取错误信息失败: {str(e)}"
    
    def _clear_last_error(self):
        if self._exports["psc_clear_last_error"](self._store) != 1: # pyright: ignore[reportCallIssue]
            raise PhiSaveCodecError("没有错误")
    
    def _free(self, ptr: int, size: int) -> None:
        if ptr == 0 or size == 0:
            return
        result = self._exports["psc_free"](self._store, ptr, size) # pyright: ignore[reportCallIssue]
        if not result:
            raise PhiSaveCodecError("内存释放失败")
        
    def _malloc(self, size: int) -> int:
        if size == 0:
            raise PhiSaveCodecError("无效的大小")
        ptr = self._exports["psc_malloc"](self._store, size) # pyright: ignore[reportCallIssue]
        if ptr == 0:
            error_msg = self._get_last_error()
            self._clear_last_error()
            raise PhiSaveCodecError(f"内存分配失败: {error_msg}")
        return ptr

    def _invoke(self, func_name: str, in_data: bytes) -> bytes:
        # 写入数据
        in_size = len(in_data)
        in_ptr = self._malloc(in_size)
        
        try:
            self._mem.write(self._store, in_data, in_ptr)
            # 调用函数
            out_size, out_ptr = self._exports["psc_"+func_name](self._store, in_ptr, in_size) # pyright: ignore[reportCallIssue]
            
            # 检查输出指针
            if out_ptr == 0:
                error_msg = self._get_last_error()
                self._clear_last_error()
                raise PhiSaveCodecError(f"函数调用失败 ({func_name}): {error_msg}")
            
            # 读取数据
            out_data = self._mem.read(self._store, out_ptr, out_ptr + out_size)
            
            # 释放临时内存
            self._free(out_ptr, out_size)
            
            return out_data
        finally:
            self._free(in_ptr, in_size)

    def _parse(self, name: str, data: bytes) -> dict:
        try:
            out = self._invoke(f"parse_{name}", data)
            return msgpack.unpackb(out, raw=False)
        except msgpack.exceptions.UnpackException as e:
            raise PhiSaveCodecError(f"MessagePack 解包失败 ({name}): {str(e)}")

    def _build(self, name: str, obj: dict) -> bytes:
        try:
            packed_data:bytes = msgpack.packb(obj, use_bin_type=True) # pyright: ignore[reportAssignmentType]
            return self._invoke(f"build_{name}", packed_data)
        except msgpack.exceptions.PackException as e:
            raise PhiSaveCodecError(f"MessagePack 打包失败 ({name}): {str(e)}")

    def memory_size(self) -> int:
        return self._mem.data_len(self._store)
    
    def parse_user(self, data: bytes) -> dict:
        return self._parse("user", data)
    
    def build_user(self, obj: dict) -> bytes:
        return self._build("user", obj)

    def parse_summary(self, data: bytes) -> dict:
        return self._parse("summary", data)
    
    def build_summary(self, obj: dict) -> bytes:
        return self._build("summary", obj)

    def parse_game_record(self, data: bytes) -> dict:
        return self._parse("game_record", data)
    
    def build_game_record(self, obj: dict) -> bytes:
        return self._build("game_record", obj)

    def parse_game_progress(self, data: bytes) -> dict:
        return self._parse("game_progress", data)
    
    def build_game_progress(self, obj: dict) -> bytes:
        return self._build("game_progress", obj)

    def parse_game_key(self, data: bytes) -> dict:
        return self._parse("game_key", data)
    
    def build_game_key(self, obj: dict) -> bytes:
        return self._build("game_key", obj)

    def parse_settings(self, data: bytes) -> dict:
        return self._parse("settings", data)
    
    def build_settings(self, obj: dict) -> bytes:
        return self._build("settings", obj)