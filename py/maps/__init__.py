
from maps.trentino import trentino_board
from maps.london import london_board
from maps.europe import europe_board

__maps={
    "trentino": trentino_board,
    "london": london_board,
    "europe": europe_board
}

def get_board(mapname:str):
    return __maps[mapname]()


def boards():
    return [x for x in __maps.keys()]