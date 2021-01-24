from board import Board, Track
from gamestate import GameState
from missions import *

b = Board()
b.add_track(Track("covent", "trafalgar", 1))
b.add_track(Track("covent", "picadilly", 1))
b.add_track(Track("covent", "british", 1))
b.add_track(Track("covent", "stpauls", 3))
b.add_track(Track("stpauls", "charterhouse", 1))
b.add_track(Track("stpauls", "bricklane", 3))
b.add_track(Track("stpauls", "toweroflondon", 3))
b.add_track(Track("stpauls", "globe", 1))
b.add_track(Track("bricklane", "toweroflondon", 3))
b.add_track(Track("bricklane", "charterhouse", 3))
b.add_track(Track("charterhouse", "kingscross", 3))
b.add_track(Track("charterhouse", "british", 4))
b.add_track(Track("british", "baker", 4))
b.add_track(Track("british", "regent", 3))
b.add_track(Track("british", "kingscross", 2))
b.add_track(Track("british", "picadilly", 2))
b.add_track(Track("kingscross", "regent", 3))
b.add_track(Track("regent", "baker", 2))
b.add_track(Track("baker", "picadilly", 4))
b.add_track(Track("baker", "hydepark", 4))
b.add_track(Track("hydepark", "buckingham", 1))
b.add_track(Track("hydepark", "picadilly", 2))
b.add_track(Track("buckingham", "picadilly", 2))
b.add_track(Track("buckingham", "bigben", 2))
b.add_track(Track("picadilly", "trafalgar", 1))
b.add_track(Track("trafalgar", "bigben", 1))
b.add_track(Track("trafalgar", "waterloo", 2))
b.add_track(Track("bigben", "waterloo", 1))
b.add_track(Track("bigben", "elephant", 3))
b.add_track(Track("waterloo", "globe", 2))
b.add_track(Track("waterloo", "elephant", 2))
b.add_track(Track("globe", "elephant", 2))
b.add_track(Track("globe", "toweroflondon", 3))
b.add_track(Track("elephant", "toweroflondon", 4))
b.add_mission(ConnectCitiesMission("baker", "toweroflondon",11))
b.add_mission(ConnectCitiesMission("kingscross", "toweroflondon",7))
b.add_mission(ConnectCitiesMission("british", "stpauls",4))
b.add_mission(ConnectCitiesMission("baker", "trafalgar",5))
b.add_mission(ConnectCitiesMission("trafalgar", "stpauls",4))
b.calculate_distances()

print(b.get_tracks_buildable_with_nrtrains(1))

print(b.min_distance_between_cities["british"])
b.export_graphviz_map("map.dot")

# sfdp -Tpng -o map.png map.dot

gamestate = GameState(b, trains_remaining=17)

max_for_routes = 0
max_for_routes_gs = None

def calculate(gs:GameState, indent = 0, exclude=set()):
    #print("  "*indent+str(gs))
    global max_for_routes, max_for_routes_gs

    # get all tracks that are possible to build
    tracks = gs.board.get_tracks_buildable_with_nrtrains(gs.trains_remaining)
    # remove from the list the tracks we already built
    tracks = tracks.difference(gs.built_tracks)
    # remove the tracks that would result in duplicates
    tracks = tracks.difference(exclude)
    # consider only the tracks that are attached to something we already built
    tracks = [t for t in tracks if (t.name1 in gs.reached_cities) or (t.name2 in gs.reached_cities)]
    if len(tracks) == 0:
        # Ran out of options! Calculate the missions :)
        gs.calculate_total_points()
        if max_for_routes < gs.total_points:
            max_for_routes_gs = gs
            max_for_routes = gs.total_points

    for t in tracks:
        exclude.add(t)
        if indent <= 4:
            print("  "*indent+"* build "+str(t))
        calculate(gs.new_with_track_built(t), indent=indent+2, exclude=set(exclude))

def calculate_start(gs:GameState):
    tracks = gs.board.get_tracks_buildable_with_nrtrains(gs.trains_remaining)
    exclude=set()
    for t in tracks:
        exclude.add(t)
        print("* build "+str(t))
        calculate(gs.new_with_track_built(t), indent=2,exclude=set(exclude))

calculate_start(gamestate)

print("Maximum points for all connected routes: "+str(max_for_routes_gs))