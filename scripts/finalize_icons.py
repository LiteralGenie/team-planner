import sys
from pathlib import Path
from typing import Generator, TypeAlias, TypedDict

from utils import (
    DRAGONTAIL_VERSION_DIR,
    GUI_ASSETS_DIR,
    GUI_BANNER_DIR,
    GUI_SPRITE_DIR,
    read_image,
    write_image,
)

# left, top, right, bot
CropArea: TypeAlias = tuple[int, int, int, int]

# pip install pillow

OUTPUT_DIR = GUI_ASSETS_DIR / "tft" / "icons"
if not OUTPUT_DIR.exists():
    print("Creating output directory", OUTPUT_DIR)
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

DEBUG_DIR = GUI_ASSETS_DIR / "tft" / "icon_debug"
if not DEBUG_DIR.exists():
    DEBUG_DIR.mkdir(exist_ok=True, parents=True)
if DEBUG_DIR.exists():
    for fp in DEBUG_DIR.glob("*"):
        fp.unlink()

# DEFAULT_CROP_AREA: CropArea = (466, 56, 466 + 350, 56 + 350)
DEFAULT_CROP_AREA: CropArea = (441, 31, 441 + 375, 31 + 375)

###


class Override(TypedDict):
    output_name: str
    fp_replacement: Path
    # left, top, right, bot
    crop: CropArea


OVERRIDES: dict[str, Override] = {
    "Alune": Override(
        output_name="Alune.png",
        fp_replacement=DRAGONTAIL_VERSION_DIR
        / "img/tft-champion/TFT11_Alune.TFT_Set11.png",
        crop=(138, 0, 238, 100),
    ),
    "Kobuko": Override(
        output_name="Kobuko.png",
        fp_replacement=DRAGONTAIL_VERSION_DIR
        / "img/tft-champion/TFT11_Kobuko.TFT_Set11.png",
        crop=(138, 0, 238, 100),
    ),
    "LeeSin": Override(
        output_name="LeeSin.png",
        fp_replacement=DRAGONTAIL_VERSION_DIR
        / "img/tft-champion/TFT11_LeeSin.TFT_Set11.png",
        crop=(138, 0, 238, 100),
    ),
    "Wukong": Override(
        output_name="WuKong.png",
        fp_replacement=DRAGONTAIL_VERSION_DIR
        / "img/tft-champion/TFT11_WuKong.TFT_Set11.png",
        crop=(138, 0, 238, 100),
    ),
    "Zoe": Override(
        output_name="Zoe.png",
        fp_replacement=DRAGONTAIL_VERSION_DIR
        / "img/tft-champion/TFT11_Zoe.TFT_Set11.png",
        crop=(138, 0, 238, 100),
    ),
}

IGNORE = ["Xayah&Rakan"]

###


def iter_sprite_banner() -> Generator[tuple[Path, Path | None], None, None]:
    for fp_sprite in GUI_SPRITE_DIR.glob("*"):
        fp_banner = GUI_BANNER_DIR / fp_sprite.name
        if not fp_banner.exists():
            fp_banner = None

        yield (fp_sprite, fp_banner)


def main():
    for fp_sprite, fp_banner in iter_sprite_banner():
        name = fp_sprite.stem
        if name in OVERRIDES:
            continue

        if not fp_banner:
            if name in IGNORE:
                continue

            raise Exception(f"No banner image or override for {fp_sprite}")

        fp_out = OUTPUT_DIR / fp_sprite.name
        print(f"Creating icon from cropped banner image for {fp_sprite.stem}")

        image = read_image(fp_banner)
        cropped = image.crop(DEFAULT_CROP_AREA)

        write_image(cropped, fp_out)

    for name, override in OVERRIDES.items():
        fp_sprite = GUI_SPRITE_DIR / (name + ".png")
        if not fp_sprite.exists():
            print(
                f"Override for {name} exists but no sprite icon exists (and therefore not in the api data). Maybe a typo in the name?"
            )
            sys.exit()

        fp_out = OUTPUT_DIR / fp_sprite.name
        print(
            f'Using override for {override["output_name"]} from {override["fp_replacement"]}'
        )

        image = read_image(override["fp_replacement"])
        cropped = image.crop(override["crop"])

        write_image(cropped, fp_out)


main()
