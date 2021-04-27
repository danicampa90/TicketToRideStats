from board import Board, Track
class GameState:
    def __init__(self, board:Board, trains_remaining: int = 0, built_tracks = [], track_points: int = 0, reached_cities = set()):
        self.board = board
        self.trains_remaining = trains_remaining
        self.built_tracks = built_tracks
        self.track_points = track_points
        self.reached_cities = reached_cities
        self.mission_points = 0
        self.total_points = 0

    def new_with_track_built(self, track:Track):
        new_tracks = self.built_tracks[:]
        new_tracks.append(track)
        new_track_points = self.track_points + GameState.assign_points(track)
        new_trains_remaining = self.trains_remaining - track.length
        new_reached_cities = self.reached_cities.union([track.name1, track.name2])
        return GameState(self.board, new_trains_remaining, new_tracks, new_track_points, new_reached_cities)

    def assign_points(track:Track):
        points_dict = {1:1, 2:2, 3:4, 4: 7, 6: 15, 8: 21} # or something like that
        return points_dict[track.length]

    def calculate_total_points(self):
        self.mission_points = 0
        for mission in self.board.missions:
            value = mission.get_value_with_tracks(self)
            if value > 0:
                self.mission_points += mission.value
        self.total_points = self.mission_points + self.track_points

    def __str__(self):
        return "<"+str(self.track_points+self.mission_points)+"pts - "+str(self.trains_remaining)+"rem - "+repr(self.built_tracks)+">"
