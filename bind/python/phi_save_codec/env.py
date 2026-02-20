from importlib.resources import files

wasm = files("phi_save_codec").joinpath("bin/phi_save_codec.wasm").read_bytes()
