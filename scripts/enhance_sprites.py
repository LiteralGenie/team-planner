from pathlib import Path
from typing import TypeAlias, TypedDict

import cv2

Image: TypeAlias = cv2.typing.MatLike

# pip install opencv-python

ROOT_DIR = Path(__file__).parent.parent
GUI_ASSETS_DIR = ROOT_DIR / "gui" / "src" / "lib" / "assets"

OUTPUT_DIR = GUI_ASSETS_DIR / "tft" / "icons"
if not OUTPUT_DIR.exists():
    print("Creating output directory", OUTPUT_DIR)
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

# https://ddragon.leagueoflegends.com/cdn/dragontail-14.8.1.tgz
DRAGONTAIL_ASSETS_DIR = GUI_ASSETS_DIR / "dragontail-14.8.1"
BIGGER_IMAGE_DIR = DRAGONTAIL_ASSETS_DIR / "img" / "champion" / "tiles"

SPRITE_DIR = GUI_ASSETS_DIR / "tft" / "sprites"

###


class BiggerImage(TypedDict):
    image: Image
    fp: Path


class ScoredCandidate(TypedDict):
    image: BiggerImage
    score: tuple


def score_candidates(
    template: Image, candidates: list[BiggerImage]
) -> list[ScoredCandidate]:
    method = cv2.TM_CCOEFF_NORMED

    scored: list[tuple[tuple, Image]] = []
    for c in candidates:
        res = cv2.matchTemplate(c["image"], template, method)
        score = cv2.minMaxLoc(res)

        scored.append(ScoredCandidate(image=c, score=score))

    def get_score(x: ScoredCandidate):
        min_val, max_val, min_loc, max_loc = x["score"]
        return max_val

    scored.sort(key=get_score, reverse=True)
    return scored


def find_splashes_for_champion(id_champion: str) -> list[BiggerImage]:
    id_champion = id_champion.lower().replace("'", "")

    matches = []
    for fp in BIGGER_IMAGE_DIR.glob("*"):
        name = str(fp.name).lower()
        if name.startswith(id_champion):
            matches.append(fp)

    results = []
    for fp in matches:
        image = cv2.imread(str(fp))
        image = preprocess_candidate(image)
        results.append(
            BiggerImage(
                image=image,
                fp=fp,
            )
        )

    return results


def preprocess_sprite(image: Image) -> Image:
    image = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY)
    return image

    # SCALE_FACTOR = 10
    # w = image.shape[0] * SCALE_FACTOR
    # h = image.shape[1] * SCALE_FACTOR
    # return cv2.resize(image, (w, h))


def preprocess_candidate(image: Image) -> Image:
    SCALE_FACTOR = 8
    w = image.shape[0] // SCALE_FACTOR
    h = image.shape[1] // SCALE_FACTOR
    image = cv2.resize(image, (w, h))
    image = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY)
    return image


def main():
    for fp_sprite in SPRITE_DIR.glob("*"):
        fp_out = OUTPUT_DIR / fp_sprite.name
        if fp_out.exists() and False:
            print(f"Skipping {fp_sprite.stem}. Icon already exists")
            continue

        id_champion = fp_sprite.stem
        splash_screens = find_splashes_for_champion(id_champion)
        if not splash_screens:
            print(f"No splashes for {id_champion} found in {BIGGER_IMAGE_DIR}")
            continue

        sprite = cv2.imread(str(fp_sprite))
        sprite = preprocess_sprite(sprite)

        scored = score_candidates(sprite, splash_screens)
        match = scored[0]
        print(
            f'Matched sprite {fp_sprite.name} to {match["image"]["fp"].name} with score {match["score"][1]:.3f} from {len(splash_screens)} candidates'
        )

        for c in scored:
            fp = OUTPUT_DIR / f'_dbg_{id_champion}_{c["score"][1]:.3f}.jpg'
            im = cv2.imread(str(c["image"]["fp"]))
            im = c["image"]["image"]
            cv2.imwrite(str(fp), im)

        cv2.imwrite(str(fp_out), cv2.imread(str(match["image"]["fp"])))

        # break


main()
