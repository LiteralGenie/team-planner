import json
import time
from typing import TypeAlias

import requests
from utils import (
    DATA_DIR,
    GUI_ASSETS_DIR,
    LATEST_SET_ID,
    download_image,
    fetch_json_cached,
    get_cdragon_asset_url,
)

USE_CACHED = True

###

DATA_URL = "https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/tfttraits.json"
DATA_FILE = DATA_DIR / "tfttraits.json"

FILTERED_DATA_FILE = GUI_ASSETS_DIR / "tft" / "tfttraits.json"

IMAGE_DIR = GUI_ASSETS_DIR / "tft" / "traits"
IMAGE_DIR.mkdir(parents=True, exist_ok=True)

###

ITrait: TypeAlias = dict
IData: TypeAlias = list[ITrait]


def fetch() -> IData:
    return fetch_json_cached(
        DATA_URL,
        DATA_FILE,
        use_cache=USE_CACHED,
    )


def download_icons(data: IData):
    for trait in data:
        url = get_cdragon_asset_url(trait["icon_path"])
        ext = str(url).split(".")[-1]

        id = trait["trait_id"]
        fp_out = IMAGE_DIR / f"{id}.{ext}"
        if fp_out.exists():
            continue

        print("Downloading", url)
        download_image(url, fp_out)
        time.sleep(1)


def is_unique(trait: ITrait) -> bool:
    styles = (effect["style_name"] for effect in trait["conditional_trait_sets"])
    return any(s == "kUnique" for s in styles)


def main():
    data = fetch()

    filtered = [d for d in data if LATEST_SET_ID in d["set"] and not is_unique(d)]
    print(f"Found {len(filtered)} traits for set {LATEST_SET_ID}")

    with open(FILTERED_DATA_FILE, "w+") as file:
        json.dump(filtered, file, indent=4)

    download_icons(filtered)


main()
