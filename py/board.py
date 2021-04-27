from queue import PriorityQueue
from missions import Mission

class Track:
    def __init__(self, name1:str, name2:str, length:int, special: object = None):
        self.name1 = name1
        self.name2 = name2
        self.length = length
        self.specials = []
        if special is None:
            self.specials = []
        elif isinstance(special, str):
            self.specials = [special]
        elif isinstance(special, list):
            self.specials = special
        else:
            raise TypeError("Expected a string or a list as 'special' argument")

    def __str__(self):
        return self.name1+("-"*self.length)+self.name2

    def __repr__(self):
        return self.name1+("-"*self.length)+self.name2


class Board:
    def __init__(self, name:str, number_of_trains:int):
        self.name = name
        self.number_of_trains = number_of_trains
        self.tracks=[]
        self.missions=[]
        self.tracks_in_city={}
        self.tracks_by_length=[None,[],[],[],[],[],[],[],[],[],[]]
        self.min_distance_between_cities={}
        self.track_scores = {1:1, 2:2, 3:4, 4: 7, 6: 15, 8: 21} # or something like that
        pass

    def set_track_scores(scores):
        self.track_scores = scores

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

    def add_mission(self, mission:Mission):
        self.missions.append(mission)

    def get_tracks_buildable_with_nrtrains(self, nr_trains: int): 
        result = set()
        if len(self.tracks_by_length) <= nr_trains:
            return set(self.tracks)

        for length in range(nr_trains, 0, -1):
            result=result.union(self.tracks_by_length[length])
        return result


    def calculate_distances(self):
        for city in self.tracks_in_city.keys():
            #print("- City:"+str(city))
            frontier = PriorityQueue()
            frontier.put((0, city))
            distances = {city: 0}
            while not frontier.empty():
                _, frontier_city = frontier.get()
                #print("  ... from "+str(frontier_city), ", which is currently at a distance of "+str(distances[frontier_city]))
                for track in self.tracks_in_city[frontier_city]:
                    other_city = track.name2 if track.name1 == frontier_city else track.name1
                    other_current_distance = distances.get(other_city, None)
                    #print("     - considering with the track "+str(track)+" ("+other_city+" distance is "+repr(other_current_distance)+")")
                    # if we don't have any distance before this or if we found a shorter path
                    if other_current_distance is None or other_current_distance > distances[frontier_city] + track.length:
                        new_distance = distances[frontier_city] + track.length
                        #print("       - setting a new shortest distance to "+ str(new_distance))
                        distances[other_city] = new_distance
                        frontier.put((new_distance, other_city))
                    pass
                pass
            pass
            self.min_distance_between_cities[city] = distances
        pass

