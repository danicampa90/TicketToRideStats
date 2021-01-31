# Ticket to ride python analysis tool

This python script analyzes the ticket to ride maps to find what is the maximum theorethical amout of points one can earn with a specified number of trains.

It's an interactive script and it doesn't have any error handling.

Once the first track is built, during the search it only builds tracks that are connected to an existing track. This is done to optimize run times.

**Notice:** This does not implement the game and is not playable. It just gather statistics on the routes and provides cool insights into it.

## Running it.

Execute in the command line:
```bash
python3 ttr.py
```

The script should prompt you what you want to do.

It also automatically exports the data of the game in CSV and the map in dot(graphviz) format. You can convert all the .dot files into png pictures by running `make`.

## To add maps
Add a file in `maps/<name>.py` and edit `__init__.py` there to import it in the list of available maps.
If you need a new ticket/mission/way of earning points added, look into missions.py

# PRs
PRs are welcome if they are licensed under a GPL compatible license.