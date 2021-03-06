from board import Board, Track
from missions import *

# a custom map on trentino (Italy). It's not available publicly(yet?)
def trentino_board():
    b = Board("trentino", 18)
    b.add_track(Track("ala", "riva", 2))
    b.add_track(Track("ala", "rovereto", 2))
    b.add_track(Track("rovereto", "pozza", 1))
    b.add_track(Track("rovereto", "pozza", 2))
    b.add_track(Track("rovereto", "riva", 1))
    b.add_track(Track("rovereto", "folgaria", 1))
    b.add_track(Track("rovereto", "trento", 2))
    b.add_track(Track("riva", "bezzecca", 1))
    b.add_track(Track("riva", "sarche", 2))
    b.add_track(Track("trento", "folgaria", 1))
    b.add_track(Track("trento", "sarche", 1))
    b.add_track(Track("trento", "cavalese", 3))
    b.add_track(Track("trento", "bolzano", 5))
    b.add_track(Track("trento", "cles", 3))
    b.add_track(Track("folgaria", "cavalese", 4))
    b.add_track(Track("sarche", "bezzecca", 4))
    b.add_track(Track("sarche", "madonna", 3))
    b.add_track(Track("madonna", "cles", 2 ))
    b.add_track(Track("cavalese", "passopordoi", 4))
    b.add_track(Track("cavalese", "bolzano", 2))
    b.add_track(Track("cles", "bolzano", 2))
    b.add_track(Track("cles", "merano", 3))
    b.add_track(Track("bolzano", "passopordoi", 3))
    b.add_track(Track("bolzano", "pontegardena", 2))
    b.add_track(Track("bolzano", "merano", 2))
    b.add_track(Track("merano", "austria-1", 3))
    b.add_track(Track("merano", "bressanone", 5))
    b.add_track(Track("pontegardena", "bressanone", 2))
    b.add_track(Track("pontegardena", "passopordoi", 2))
    b.add_track(Track("passopordoi", "brunico", 3))
    b.add_track(Track("brunico", "austria-3", 3))
    b.add_track(Track("brunico", "bressanone", 2))
    b.add_track(Track("bressanone", "austria-2", 4))
    b.add_mission(ConnectCitiesMission("madonna","austria-1", 10))
    b.add_mission(ConnectCitiesMission("bressanone","sarche", 11))
    b.add_mission(ConnectCitiesMission("brunico","cles", 9))
    b.add_mission(ConnectCitiesMission("sarche","rovereto", 3))
    b.add_mission(ConnectCitiesMission("pontegardena","ala", 11))
    b.add_mission(ConnectCitiesMission("bressanone","cavalese", 7))
    b.add_mission(ConnectCitiesMission("passopordoi","pozza", 11))
    b.add_mission(ConnectCitiesMission("merano","rovereto", 9))
    b.add_mission(ConnectCitiesMission("pontegardena","austria-2", 6))
    b.add_mission(ConnectCitiesMission("trento","austria-1", 10))
    b.add_mission(ConnectCitiesMission("cavalese","bezzecca", 7))
    b.add_mission(ConnectCitiesMission("cles","ala", 7))
    b.add_mission(ConnectCitiesMission("madonna","trento", 5))
    b.add_mission(ConnectCitiesMission("folgaria","riva", 3))
    b.add_mission(ConnectCitiesMission("folgaria","brunico", 12))
    b.add_mission(ConnectCitiesMission("merano","passopordoi", 6))
    b.add_mission(ConnectCitiesMission("bezzecca","pozza", 5))
    b.add_mission(ConnectCitiesMission("bolzano","trento", 4))
    b.add_mission(ConnectCitiesMission("cavalese","ala", 7))
    b.add_mission(ConnectCitiesMission("bolzano","riva", 8))
    return b
