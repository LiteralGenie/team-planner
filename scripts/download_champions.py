import json
import time
from typing import TypeAlias

import requests
from utils import (
    CDRAGON_URL,
    DATA_DIR,
    GUI_ASSETS_DIR,
    LATEST_SET_ID,
    LATEST_SET_NAME,
    LATEST_SET_PREFIX,
    download_image,
    fetch_json_cached,
    get_cdragon_asset_url,
)

USE_CACHED = True

###

TEAM_PLANNER_DATA_URL = (
    CDRAGON_URL
    / "plugins/rcp-be-lol-game-data/global/default/v1/tftchampions-teamplanner.json"
)
TEAM_PLANNER_DATA_FILE = DATA_DIR / "tftchampions-teamplanner.json"

SET_DATA_URL = CDRAGON_URL / "cdragon/tft/en_us.json"
SET_DATA_FILE = DATA_DIR / "en_us.json"

CHARACTERS_DATA_URL = CDRAGON_URL / "game/data/tftteamplanner/characters.bin.json"
CHARACTERS_DATA_FILE = DATA_DIR / "characters.bin.json"

MERGED_DATA_FILE = GUI_ASSETS_DIR / "tft" / "merged_teamplanner_data.json"

ICON_DIR = GUI_ASSETS_DIR / "tft" / "champions"
ICON_DIR.mkdir(parents=True, exist_ok=True)

SPLASH_DIR = GUI_ASSETS_DIR / "tft" / "champion_splashes"
SPLASH_DIR.mkdir(parents=True, exist_ok=True)

###

ITeamPlannerData: TypeAlias = list[dict]
ISetData: TypeAlias = dict
ICharactersData: TypeAlias = dict
IMergedData: TypeAlias = list[dict]


def fetch_teamplanner_data() -> ITeamPlannerData:
    return fetch_json_cached(
        TEAM_PLANNER_DATA_URL,
        TEAM_PLANNER_DATA_FILE,
        use_cache=USE_CACHED,
    )


def fetch_unit_data() -> ISetData:
    return fetch_json_cached(
        SET_DATA_URL,
        SET_DATA_FILE,
        use_cache=USE_CACHED,
    )


def fetch_characters_data() -> ISetData:
    return fetch_json_cached(
        CHARACTERS_DATA_URL,
        CHARACTERS_DATA_FILE,
        use_cache=USE_CACHED,
    )


def filter_team_planner_data(tp_data: ITeamPlannerData) -> ITeamPlannerData:
    return [d for d in tp_data if LATEST_SET_PREFIX in d["character_id"]]


def build_merged_data(
    tp_data: ITeamPlannerData,
    set_data: ISetData,
    char_data: ICharactersData,
) -> IMergedData:
    """
    Append the stats (ad, range, etc) from set data to the team planner data for each champion
    """

    filtered = filter_team_planner_data(tp_data)

    latest_set = next(
        s for s in set_data["sets"].values() if s["name"] == LATEST_SET_NAME
    )

    merged: IMergedData = []
    for c in filtered:
        unit = next(
            u
            for u in latest_set["champions"]
            if u["characterName"] == c["character_id"]
        )

        role = find_role_for_champion(c["character_id"], char_data)

        merged.append(
            dict(
                # from teamplanner data
                character_id=c["character_id"],
                tier=c["tier"],
                display_name=c["display_name"],
                traits=c["traits"],
                # from set data
                stats=unit["stats"],
                ability=unit["ability"],
                # from character data
                damage_type=get_damage_type(role),
            )
        )

    return merged


def find_role_for_champion(id: str, char_data: ICharactersData) -> dict:
    char = next(c for c in char_data.values() if c.get("mCharacterName") == id)
    role = next(
        data for name, data in char_data.items() if name == char["CharacterRole"]
    )
    return role


def get_damage_type(role: dict) -> dict:
    if role["name"].startswith("AP"):
        return dict(
            is_ad=False,
            is_ap=True,
        )
    elif role["name"].startswith("AD"):
        return dict(
            is_ad=True,
            is_ap=False,
        )
    else:
        raise Exception(role["name"])


# def infer_damage_type(set_data):
#     is_ad = "<physicalDamage>" in set_data["ability"]["desc"]
#     is_ap = "<magicDamage>" in set_data["ability"]["desc"]

#     return dict(is_ad=is_ad, is_ap=is_ap,)


def download_icons(data: ITeamPlannerData):
    filtered = filter_team_planner_data(data)

    for champion in filtered:
        url = get_cdragon_asset_url(champion["squareIconPath"])
        ext = str(url).split(".")[-1]

        id = champion["character_id"]
        fp_out = ICON_DIR / f"{id}.{ext}"
        if fp_out.exists():
            continue

        print("Downloading icon", url)
        download_image(url, fp_out)
        time.sleep(1)


def download_splashes(data: ITeamPlannerData):
    filtered = filter_team_planner_data(data)

    for champion in filtered:
        url = get_cdragon_asset_url(champion["squareSplashIconPath"])
        ext = str(url).split(".")[-1]

        id = champion["character_id"]
        fp_out = SPLASH_DIR / f"{id}.{ext}"
        if fp_out.exists():
            continue

        print("Downloading splash", url)
        download_image(url, fp_out)
        time.sleep(1)


def main():
    tp_data = fetch_teamplanner_data()
    set_data = fetch_unit_data()
    char_data = fetch_characters_data()

    merged = build_merged_data(tp_data, set_data, char_data)
    print(f"Found {len(merged)} champions for set {LATEST_SET_ID}")

    download_icons(tp_data)
    download_splashes(tp_data)

    print("Creating", MERGED_DATA_FILE)
    with open(MERGED_DATA_FILE, "w+") as file:
        json.dump(merged, file, indent=4)


main()
