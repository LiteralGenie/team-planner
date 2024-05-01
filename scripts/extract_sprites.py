import json
from pathlib import Path

from utils import (
    CHAMPION_DATA_FILE,
    CHAMPION_SPRITE_DIR,
    GUI_SPRITE_DIR,
    read_image,
    write_image,
)

# pip install pillow


OUTPUT_DIR = GUI_SPRITE_DIR
if not OUTPUT_DIR.exists():
    print("Creating output directory", OUTPUT_DIR)
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)


###


def clean_name(name: str) -> str:
    return name.replace(" ", "")


with open(CHAMPION_DATA_FILE) as file:
    champion_data = json.load(file)

for champion in champion_data["data"].values():
    if champion["id"].startswith("TFTTutorial_"):
        continue

    d = champion["image"]
    name = clean_name(champion["name"])

    fp_image: Path = CHAMPION_SPRITE_DIR / d["sprite"]
    fp_out = OUTPUT_DIR / (name + ".png")
    print(f"Cropping {fp_image} to {fp_out}")

    left = d["x"]
    top = d["y"]
    right = d["x"] + d["w"]
    bot = d["y"] + d["h"]

    sprite_sheet = read_image(fp_image)
    cropped = sprite_sheet.crop((left, top, right, bot))
    write_image(cropped, fp_out)
