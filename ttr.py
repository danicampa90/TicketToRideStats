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
    print(city_names)

# sfdp -Tpng -o map.png map.dot
#winning_gs = OptimalStrategyCalculator().calculate_from_board(b, b.number_of_trains)
#print("Maximum points for all connected routes: "+str(winning_gs))

