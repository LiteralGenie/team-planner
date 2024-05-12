import shutil
import time

from lib.utils import CDRAGON_URL, DATA_DIR, GUI_ASSETS_DIR, download_image
from PIL import Image
from yarl import URL

OUT_DIR = GUI_ASSETS_DIR / "tft" / "misc"
OUT_DIR.mkdir(exist_ok=True, parents=True)

targets: list[dict] = [
    dict(
        name="statmodsabilitypowericon.png",
        url=CDRAGON_URL / "game/assets/perks/statmods/statmodsabilitypowericon.png",
        crop=(6, 6, 26, 26),
    ),
    dict(
        name="statmodsattackdamageicon.png",
        url=CDRAGON_URL / "game/assets/perks/statmods/statmodsattackdamageicon.png",
        crop=(6, 6, 26, 26),
    ),
    dict(
        name="statmodshealthscalingicon.png",
        url=CDRAGON_URL / "game/assets/perks/statmods/statmodshealthscalingicon.png",
        crop=(6, 6, 26, 26),
    ),
    dict(
        name="statmodsarmoricon.png",
        url=CDRAGON_URL / "game/assets/perks/statmods/statmodsarmoricon.png",
        crop=(6, 6, 26, 26),
    ),
    dict(
        name="statmodsmagicresicon.png",
        url=CDRAGON_URL / "game/assets/perks/statmods/statmodsmagicresicon.png",
        crop=(6, 6, 26, 26),
    ),
    dict(
        name="statmodsattackspeedicon.png",
        url=CDRAGON_URL / "game/assets/perks/statmods/statmodsattackspeedicon.png",
        crop=(6, 6, 26, 26),
    ),
    dict(
        name="mana.png",
        url=CDRAGON_URL / "game/assets/ux/fonts/texticons.png",
        crop=(1, 2, 21, 22),
    ),
    dict(
        name="crit.png",
        url=CDRAGON_URL / "game/assets/ux/fonts/texticons.png",
        crop=(72, 48, 92, 68),
    ),
]

for tgt in targets:
    url: URL = tgt["url"]
    filename = tgt["name"]
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
