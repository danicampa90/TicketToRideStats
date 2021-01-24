from board import Board, Track

b = Board()
b.add_track(Track("kobenhavn", "stockholm", 3))
b.add_track(Track("kobenhavn", "essen", 3))
b.add_track(Track("amsterdam", "essen", 3))
b.add_track(Track("amsterdam", "london", 3))
b.add_track(Track("berlin", "essen", 2))
b.add_track(Track("berlin", "frankfurt", 3))
b.add_track(Track("essen", "frankfurt", 2))
b.add_track(Track("edinburgh", "london", 4))
b.add_track(Track("amsterdam", "bruxelles", 1))

print(b.get_tracks_buildable_with_nrtrains(3))