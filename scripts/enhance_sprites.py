from pathlib import Path
from typing import TypeAlias, TypedDict

from img2vec_pytorch import Img2Vec
from PIL import Image
from sklearn.metrics.pairwise import cosine_similarity
from utils import (
    CENTERED_BANNER_DIR,
    GUI_ASSETS_DIR,
    GUI_BANNER_DIR,
    GUI_SPRITE_DIR,
    IImage,
    read_image,
    write_image,
)

# pip install img2vec_pytorch pillow

OUTPUT_DIR = GUI_BANNER_DIR
if not OUTPUT_DIR.exists():
    print("Creating output directory", OUTPUT_DIR)
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

DEBUG_DIR = GUI_ASSETS_DIR / "tft" / "banner_debug"
if not DEBUG_DIR.exists():
    DEBUG_DIR.mkdir(exist_ok=True, parents=True)
if DEBUG_DIR.exists():
    for fp in DEBUG_DIR.glob("*"):
        fp.unlink()


###


class BiggerImage(TypedDict):
    image: IImage
    fp: Path


class ScoredCandidate(TypedDict):
    image: BiggerImage
    score: float


img2vec = Img2Vec(cuda=False)


# https://github.com/christiansafka/img2vec/blob/master/example/test_img_similarity.py
def score_one(template, candidate) -> float:
    sim = cosine_similarity(template.reshape((1, -1)), candidate.reshape((1, -1)))
    return sim[0][0]


def score_candidates(
    template: IImage, candidates: list[BiggerImage]
) -> list[ScoredCandidate]:
    template_features = img2vec.get_vec(template, tensor=True)

    scored: list[ScoredCandidate] = []
    for c in candidates:
        features = img2vec.get_vec(c["image"], tensor=True)
        score = score_one(template_features, features)

        scored.append(ScoredCandidate(image=c, score=score))

    scored.sort(key=lambda c: c["score"], reverse=True)
    return scored


def find_splashes_for_champion(id_champion: str) -> list[BiggerImage]:
    id_champion = id_champion.lower().replace("'", "")

    matches = []
    for fp in CENTERED_BANNER_DIR.glob("*"):
        name = str(fp.name).lower()
        if name.startswith(id_champion):
            matches.append(fp)

    results = []
    for fp in matches:
        image = read_image(fp)
        image = preprocess_candidate(image)
        results.append(
            BiggerImage(
                image=image,
                fp=fp,
            )
        )

    return results


def preprocess_sprite(image: IImage) -> IImage:
    return image


def preprocess_candidate(image: IImage) -> IImage:
    return image


def main():
    for fp_sprite in GUI_SPRITE_DIR.glob("*"):
        fp_out = OUTPUT_DIR / fp_sprite.name
        if fp_out.exists() and False:
            print(f"Skipping {fp_sprite.stem}. Icon already exists")
            continue

        id_champion = fp_sprite.stem
        splash_screens = find_splashes_for_champion(id_champion)
        if not splash_screens:
            print(f"No splashes for {id_champion} found in {CENTERED_BANNER_DIR}")
            continue

        sprite = read_image(fp_sprite)
        sprite = preprocess_sprite(sprite)

        scored = score_candidates(sprite, splash_screens)
        match = scored[0]
        print(
            f'Matched sprite {fp_sprite.name} to {match["image"]["fp"].name} with score {match["score"]:.6f} from {len(splash_screens)} candidates'
        )

        for c in scored:
            fp = DEBUG_DIR / f'{id_champion}_{c["score"]:.6f}.jpg'
            im = read_image(c["image"]["fp"])
            im = c["image"]["image"]
            write_image(im, fp)

        write_image(read_image(match["image"]["fp"]), fp_out)

        # break


main()
