import shutil
from pathlib import Path
from typing import TypeAlias

import requests
from PIL import Image
from yarl import URL

IImage: TypeAlias = Image.Image

LATEST_SET_ID = "TFTSet11"
LATEST_SET_PREFIX = "TFT11"

ROOT_DIR = Path(__file__).parent.parent

DATA_DIR = ROOT_DIR / "data"

GUI_ASSETS_DIR = ROOT_DIR / "gui" / "src" / "lib" / "assets"

CDRAGON_URL = URL("https://raw.communitydragon.org") / "latest"


def read_image(fp: str | Path) -> IImage:
    return Image.open(fp)


def write_image(image: IImage, fp: str | Path):
    image.save(fp)


# https://www.communitydragon.org/documentation/assets
def get_cdragon_asset_url(raw: str) -> URL:
    raw_prefix = "/lol-game-data/assets/"
    real_prefix = "/plugins/rcp-be-lol-game-data/global/default/"

    path = raw
    path = path.lower()
    path = path.replace(raw_prefix, real_prefix)
    return CDRAGON_URL / path


def download_image(url: URL | str, fp: Path):
    resp = requests.get(str(url), stream=True)
    if resp.status_code != 200:
        raise Exception(str(resp))

    with open(fp, "wb") as file:
        resp.raw.decode_content = True
        shutil.copyfileobj(resp.raw, file)
