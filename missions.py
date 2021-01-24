
class Mission:
    def get_value_with_tracks(self, gs):
        pass

class ConnectCitiesMission(Mission):
    def __init__(self, name1: str, name2: str, value: int):
        self.name1 = name1
        self.name2 = name2
        self.value = value

    def get_value_with_tracks(self, gs):
        if self.name1 in gs.reached_cities and self.name2 in gs.reached_cities:
            return self.value
        else:
            return -self.value
            