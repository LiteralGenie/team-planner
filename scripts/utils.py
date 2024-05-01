from pathlib import Path
from typing import TypeAlias

from PIL import Image

IImage: TypeAlias = Image.Image


ROOT_DIR = Path(__file__).parent.parent
GUI_ASSETS_DIR = ROOT_DIR / "gui" / "src" / "lib" / "assets"
GUI_SPRITE_DIR = GUI_ASSETS_DIR / "tft" / "sprites"
GUI_BANNER_DIR = GUI_ASSETS_DIR / "tft" / "banners"

# https://ddragon.leagueoflegends.com/cdn/dragontail-14.8.1.tgz
DRAGONTAIL_ASSETS_DIR = GUI_ASSETS_DIR / "dragontail-14.8.1"
CENTERED_BANNER_DIR = DRAGONTAIL_ASSETS_DIR / "img" / "champion" / "centered"

DRAGONTAIL_VERSION_DIR = DRAGONTAIL_ASSETS_DIR / "14.8.1"
CHAMPION_SPRITE_DIR = DRAGONTAIL_VERSION_DIR / "img" / "sprite"
CHAMPION_DATA_FILE = DRAGONTAIL_VERSION_DIR / "data" / "en_US" / "tft-champion.json"


def read_image(fp: str | Path) -> IImage:
    return Image.open(fp)


def write_image(image: IImage, fp: str | Path):
    image.save(fp)
