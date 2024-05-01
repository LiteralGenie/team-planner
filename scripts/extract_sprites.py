import json
from pathlib import Path

import cv2

# pip install opencv-python

ROOT_DIR = Path(__file__).parent.parent
GUI_ASSETS_DIR = ROOT_DIR / "gui" / "src" / "lib" / "assets"

OUTPUT_DIR = GUI_ASSETS_DIR / "tft" / "sprites"
if not OUTPUT_DIR.exists():
    print("Creating output directory", OUTPUT_DIR)
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

# https://ddragon.leagueoflegends.com/cdn/dragontail-14.8.1.tgz
DRAGONTAIL_ASSETS_DIR = GUI_ASSETS_DIR / "dragontail-14.8.1"
CHAMPION_SPRITE_DIR = DRAGONTAIL_ASSETS_DIR / "14.8.1" / "img" / "sprite"
CHAMPION_DATA_FILE = (
    DRAGONTAIL_ASSETS_DIR / "14.8.1" / "data" / "en_US" / "tft-champion.json"
)

###

with open(CHAMPION_DATA_FILE) as file:
    champion_data = json.load(file)

for champion in champion_data["data"].values():
    if champion["id"].startswith("TFTTutorial_"):
        continue

    d = champion["image"]

    fp_image: Path = CHAMPION_SPRITE_DIR / d["sprite"]
    fp_out = OUTPUT_DIR / (champion["name"] + fp_image.suffix)
    print(f"Cropping {fp_image} to {fp_out}")

    left = d["x"]
    top = d["y"]
    right = d["x"] + d["w"]
    bot = d["y"] + d["h"]

    sprite_sheet = cv2.imread(str(fp_image))
    cropped = sprite_sheet[top:bot, left:right]
    cv2.imwrite(str(fp_out), cropped)
