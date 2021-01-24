from gamestate import GameState
from london import london_board
from trentino import trentino_board

b = trentino_board()
b.calculate_distances()

print(b.get_tracks_buildable_with_nrtrains(1))

b.export_graphviz_map("map.dot")

# sfdp -Tpng -o map.png map.dot

gamestate = GameState(b, trains_remaining=18)

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