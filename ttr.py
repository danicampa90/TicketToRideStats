from gamestate import GameState
from maps import boards, get_board
from optimal_strategy import OptimalStrategyCalculator
from export import export_all_csv, export_graphviz_map

for boardname in boards():
    b = get_board(boardname)
    b.calculate_distances()

    export_graphviz_map(b)
    export_all_csv(b)

    city_names = list(b.tracks_in_city.keys())
    city_names.sort()
    print("Cities in",b.name,":",','.join(city_names))


print("Which map you want to load amongst those?")
for mapname in boards():
    print("-", mapname)
map = input("map name: ")
b = get_board(map)

print("Usually there are",b.number_of_trains,"trains in this map.")
print("How many trains do you want to use? Note that anything bigger than 20/25 trains takes a long time (exponentially increasing...).")
nrtrains = int(input("Number of trains:"))
print("Calculating...")
winning_gs = OptimalStrategyCalculator().calculate_from_board(b, nrtrains)
print("Maximum points can be achieved with these tracks: "+str(winning_gs))
print("(excluding the longest route bonus, but keeping into account missions etc)")

