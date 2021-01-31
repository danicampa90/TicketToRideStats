from gamestate import GameState
from maps import boards, get_board
from optimal_strategy import OptimalStrategyCalculator


b = get_board(boards()[0])
b.calculate_distances()

print(b.get_tracks_buildable_with_nrtrains(1))

b.export_graphviz_map("map.dot")

# sfdp -Tpng -o map.png map.dot
winning_gs = OptimalStrategyCalculator().calculate_from_board(b, 18)
print("Maximum points for all connected routes: "+str(winning_gs))