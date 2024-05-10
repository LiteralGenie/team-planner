import time

from lib.utils import CDRAGON_URL, DATA_DIR, GUI_ASSETS_DIR, download_image
from PIL import Image
from yarl import URL

OUT_DIR = GUI_ASSETS_DIR / "tft" / "misc"
OUT_DIR.mkdir(exist_ok=True, parents=True)

targets: list[dict] = [
    dict(
        url=CDRAGON_URL / "game/assets/perks/statmods/statmodsabilitypowericon.png",
        crop=(6, 6, 26, 26),
    ),
    dict(
        url=CDRAGON_URL / "game/assets/perks/statmods/statmodsattackdamageicon.png",
        crop=(6, 6, 26, 26),
    ),
]

for tgt in targets:
    url: URL = tgt["url"]
    filename = url.parts[-1]
    print(filename)

    fp_raw = DATA_DIR / filename
    if not fp_raw.exists():
        print(f"Downloading from {url}")
        download_image(url, fp_raw)

    fp_cropped = OUT_DIR / filename
    if crop := tgt.get("crop"):
        img = Image.open(fp_raw)
        img = img.crop(crop)
        img.save(fp_cropped)

    time.sleep(1)
