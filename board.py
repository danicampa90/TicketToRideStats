class Track:
    def __init__(self, name1:str, name2:str, length:int):
        self.name1 = name1
        self.name2 = name2
        self.length = length

    def __str__(self):
        return self.name1+("-"*self.length)+self.name2

    def __repr__(self):
        return self.name1+("-"*self.length)+self.name2


class Board:
    def __init__(self):
        self.tracks=[]
        self.tracks_in_city={}
        self.tracks_by_length=[None,[],[],[],[],[],[],[],[],[],[]]
        pass

    def add_track(self, track: Track):
        self.tracks.append(track)

        # add to city 1's tracks
        l = self.tracks_in_city.get(track.name1, [])
        l.append(track)
        self.tracks_in_city[track.name1] = l
        # add to city 2's tracks
        l = self.tracks_in_city.get(track.name2, [])
        l.append(track)
        self.tracks_in_city[track.name2] = l
        # add to tracks_by_length
        self.tracks_by_length[track.length].append(track)
        pass

    def get_tracks_buildable_with_nrtrains(self, nr_trains: int): 
        result = []
        for len in range(nr_trains, 0, -1):
            result.extend(self.tracks_by_length[len])
        return result


