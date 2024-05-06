import json
import shutil
from pathlib import Path
from typing import TypeAlias

import requests
from PIL import Image
from yarl import URL

IImage: TypeAlias = Image.Image

LATEST_SET_ID = "TFTSet11"
LATEST_SET_PREFIX = "TFT11"
LATEST_SET_NAME = "Set11"

ROOT_DIR = Path(__file__).parent.parent

DATA_DIR = ROOT_DIR / "data"

GUI_ASSETS_DIR = ROOT_DIR / "gui" / "src" / "lib" / "assets"

CDRAGON_URL = URL("https://raw.communitydragon.org") / "latest"


def read_image(fp: str | Path) -> IImage:
    return Image.open(fp)


def write_image(image: IImage, fp: str | Path):
    image.save(fp)


# https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/assets/characters/tft11_ahri/hud/tft11_ahri_square.tft_set11.jpg
# https://raw.communitydragon.org/latest/lol-game-data/assets/assets/characters/tft11_aatrox/hud/tft11_aatrox_square.tft_set11.jpg


#                        /lol-game-data/assets/ASSETS/UX/TFT/ChampionSplashes/TFT11_Ahri_Mobile.TFT_Set11.png
# /plugins/rcp-be-lol-game-data/global/default/assets/ux/tft/championsplashes/tft11_ahri_mobile.tft_set11.png

# /lol-game-data/assets/ASSETS/Characters/TFT11_Ahri/HUD/TFT11_Ahri_Square.TFT_Set11.jpg
# /plugins/rcp-be-lol-game-data/global/default/assets/characters/tft11_ahri/hud/tft11_ahri_square.tft_set11.jpg


# https://www.communitydragon.org/documentation/assets
def get_cdragon_asset_url(raw: str) -> URL:
    raw_prefix = "/lol-game-data/assets/"
    real_prefix = "/plugins/rcp-be-lol-game-data/global/default/"

    path = raw
    path = path.lower()
    path = path.replace(raw_prefix, real_prefix)
    path = path.strip("/")
    return CDRAGON_URL / path


def download_image(url: URL | str, fp: Path):
    resp = requests.get(str(url), stream=True)
    if resp.status_code != 200:
        raise Exception(str(resp))

    with open(fp, "wb") as file:
        resp.raw.decode_content = True
        shutil.copyfileobj(resp.raw, file)


def fetch_json_cached(url: URL | str, fp: Path, use_cache=True):
    if use_cache and fp.exists():
        with open(fp) as file:
            return json.load(file)
    else:
        data = requests.get(str(url)).json()
        print(f"Fetching {url}")

        with open(fp, "w+") as file:
            json.dump(data, file, indent=4)
        return data
