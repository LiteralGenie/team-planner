import json
import time
from typing import TypeAlias

import requests
from utils import (
    DATA_DIR,
    GUI_ASSETS_DIR,
    LATEST_SET_ID,
    LATEST_SET_PREFIX,
    download_image,
    get_cdragon_asset_url,
)

USE_CACHED = True

###

DATA_URL = "https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/tftchampions.json"
DATA_FILE = DATA_DIR / "tftchampions.json"

FILTERED_DATA_FILE = GUI_ASSETS_DIR / "tft" / "tftchampions.json"

IMAGE_DIR = GUI_ASSETS_DIR / "tft" / "champions"
IMAGE_DIR.mkdir(parents=True, exist_ok=True)

###

IData: TypeAlias = list[dict]


def fetch() -> IData:
    data = requests.get(DATA_URL).json()

    if USE_CACHED and DATA_FILE.exists():
        with open(DATA_FILE) as file:
            return json.load(file)
    else:
        data = requests.get(DATA_URL).json()
        print(f"Fetching {DATA_URL}")

        with open(DATA_FILE, "w+") as file:
            json.dump(data, file, indent=4)
        return data


def download_icons(data: IData):
    for champion in data:
        record = champion["character_record"]
        url = get_cdragon_asset_url(record["squareIconPath"])

        id = record["character_id"]
        ext = url.split(".")[-1]
        fp_out = IMAGE_DIR / f"{id}.{ext}"
        if fp_out.exists():
            continue

        print("Downloading", url)
        download_image(url, fp_out)
        time.sleep(1)


def main():
    data = fetch()

    filtered = [d for d in data if LATEST_SET_PREFIX in d["name"]]
    print(f"Found {len(filtered)} champions for set {LATEST_SET_ID}")

    with open(FILTERED_DATA_FILE, "w+") as file:
        json.dump(filtered, file, indent=4)

    download_icons(filtered)


main()
