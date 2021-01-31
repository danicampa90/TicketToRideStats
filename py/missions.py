
class Mission:
    def get_value_with_tracks(self, gs):
        pass
    def mission_type(self):
        pass
    def mission_spec(self):
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

    def mission_type(self):
        return "connect_cities"

    def mission_spec(self):
        return self.name1+","+self.name2+","+str(self.value)


            
class ConnectDistrictsMission(Mission):
    def __init__(self, city_names, value: int):
        self.city_names = city_names
        self.value = value

    def get_value_with_tracks(self, gs):
        for city_name in self.city_names:    
            if city_name not in gs.reached_cities:
                return 0
        return self.value
            
    def mission_type(self):
        return "connect_district"

    def mission_spec(self):
        return ''+'|'.join(self.city_names)+','+str(self.value)