from gamestate import GameState
from board import Board

class OptimalStrategyCalculator:
    def __init__(self):
        self.max_for_routes = 0
        self.max_for_routes_gs = None

    def __calculate_recursive(self, gs:GameState, indent = 0, exclude=set()):
        #print("  "*indent+str(gs))

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
            if self.max_for_routes < gs.total_points:
                self.max_for_routes_gs = gs
                self.max_for_routes = gs.total_points

        for t in tracks:
            exclude.add(t)
            if indent <= 3:
                print("  "*indent+"* build "+str(t))
            self.__calculate_recursive(gs.new_with_track_built(t), indent=indent+2, exclude=set(exclude))

    def calculate_from_state(self, gs:GameState):
        self.max_for_routes = 0
        self.max_for_routes_gs = None
        tracks = gs.board.get_tracks_buildable_with_nrtrains(gs.trains_remaining)
        exclude=set()
        for t in tracks:
            exclude.add(t)
            print("* build "+str(t))
            self.__calculate_recursive(gs.new_with_track_built(t), indent=2,exclude=set(exclude))
        return self.max_for_routes_gs

    def calculate_from_board(self, board, trains):
        gamestate = GameState(board, trains_remaining=trains)
        return self.calculate_from_state(gamestate)
        