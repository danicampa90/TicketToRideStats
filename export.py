from board import Board

def export_all_csv(board: Board):
    with open(board.name+"_tracks.csv","w") as fout:
        fout.write("name1,name2,length\n")
        for track in board.tracks:
            fout.write(track.name1+","+track.name2+","+str(track.length)+"\n")
    with open(board.name+"_tickets.csv","w") as fout:
        fout.write('type,"spec in next columns"\n')
        for miss in board.missions:
            fout.write(miss.mission_type()+","+miss.mission_spec()+"\n")
    with open(board.name+"_metadata.csv","w") as fout:
        fout.write('category,key,value\n')
        fout.write("player,trains_number"+str(board.number_of_trains)+"\n")
        for length,points in board.track_scores.items():
            fout.write("trackscores,"+str(length)+","+str(points)+"\n")

def export_graphviz_map(board:Board):
    with open(board.name+".dot","w") as dotfile:
        dotfile.write("graph {\n")
        for track in board.tracks:
            dotfile.write("  "+track.name1+"--"+track.name2+"[label="+str(track.length)+"];\n")
        
        dotfile.write("}")
        

