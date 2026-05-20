from pathlib import Path
import shutil

src = Path("../../output/phi_save_codec.wasm")
dst = Path("./phi_save_codec/bin/phi_save_codec.wasm")

dst.parent.mkdir(parents=True, exist_ok=True)
shutil.copy2(src, dst)