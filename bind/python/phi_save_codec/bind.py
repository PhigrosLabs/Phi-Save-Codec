import msgpack
from wasmtime import Store, Module, Instance, Engine, Memory
from phi_save_codec.error import PhiSaveCodecError


class PhiSaveCodec:
    def __init__(self, wasm: None | bytes = None):
        if wasm is None:
            from phi_save_codec.env import wasm as wasm
        self._engine = Engine()
        self._store = Store(self._engine)
        self._module = Module(self._engine, wasm)
        self._instance = Instance(self._store, self._module, [])
        self._exports = self._instance.exports(self._store)
        self._mem: Memory = self._exports["memory"]  # pyright: ignore[reportAttributeAccessIssue]

    def _free(self, ptr: int, size: int) -> None:
        if ptr == 0 or size == 0:
            return
        result = self._exports["psc_free"](self._store, ptr, size)  # pyright: ignore[reportCallIssue]
        if not result:
            raise PhiSaveCodecError("内存释放失败")

    def _malloc(self, size: int) -> int:
        if size == 0:
            raise PhiSaveCodecError("无效的大小")
        ptr = self._exports["psc_malloc"](self._store, size)  # pyright: ignore[reportCallIssue]
        if ptr == 0:

            raise PhiSaveCodecError("内存分配失败")
        return ptr

    def _invoke(self, func_name: str, in_data: bytes) -> bytes:
        in_size = len(in_data)
        in_ptr = self._malloc(in_size)

        try:
            self._mem.write(self._store, in_data, in_ptr)

            tag, out_size, out_ptr = self._exports["psc_" + func_name](
                self._store, in_ptr, in_size
            ) # pyright: ignore[reportCallIssue]

            if tag != 0:
                if out_ptr != 0 and out_size != 0:
                    err_bytes = self._mem.read(self._store, out_ptr, out_ptr + out_size)
                    try:
                        msg = err_bytes.decode("utf-8")
                    except Exception:
                        msg = "unknown error (invalid utf-8)"
                    self._free(out_ptr, out_size)
                else:
                    msg = "unknown error (empty error payload)"

                raise PhiSaveCodecError(f"{func_name}: {msg}")

            if out_ptr == 0 or out_size == 0:
                raise PhiSaveCodecError(f"{func_name}: empty result")

            out_data = self._mem.read(self._store, out_ptr, out_ptr + out_size)
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
            packed_data: bytes = msgpack.packb(obj, use_bin_type=True)  # pyright: ignore[reportAssignmentType]
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
