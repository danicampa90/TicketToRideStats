from board import Board, Track
from missions import *

# ticket to ride london map data
def london_board():
    b = Board("london", 17)
    b.add_track(Track("covent", "trafalgar", 1))
    b.add_track(Track("covent", "picadilly", 1))
    b.add_track(Track("covent", "british", 1))
    b.add_track(Track("covent", "stpauls", 3))
    b.add_track(Track("stpauls", "charterhouse", 1))
    b.add_track(Track("stpauls", "bricklane", 3))
    b.add_track(Track("stpauls", "tower", 3))
    b.add_track(Track("stpauls", "globe", 1))
    b.add_track(Track("bricklane", "tower", 3))
    b.add_track(Track("bricklane", "charterhouse", 3))
    b.add_track(Track("charterhouse", "kingscross", 3))
    b.add_track(Track("charterhouse", "british", 4))
    b.add_track(Track("british", "baker", 4))
    b.add_track(Track("british", "regent", 3))
    b.add_track(Track("british", "kingscross", 2))
    b.add_track(Track("british", "picadilly", 2))
    b.add_track(Track("kingscross", "regent", 3))
    b.add_track(Track("regent", "baker", 2))
    b.add_track(Track("baker", "picadilly", 4))
    b.add_track(Track("baker", "hydepark", 4))
    b.add_track(Track("hydepark", "buckingham", 1))
    b.add_track(Track("hydepark", "picadilly", 2))
    b.add_track(Track("buckingham", "picadilly", 2))
    b.add_track(Track("buckingham", "bigben", 2))
    b.add_track(Track("picadilly", "trafalgar", 1))
    b.add_track(Track("trafalgar", "bigben", 1))
    b.add_track(Track("trafalgar", "waterloo", 2))
    b.add_track(Track("bigben", "waterloo", 1))
    b.add_track(Track("bigben", "elephant", 3))
    b.add_track(Track("waterloo", "globe", 2))
    b.add_track(Track("waterloo", "elephant", 2))
    b.add_track(Track("globe", "elephant", 2))
    b.add_track(Track("globe", "tower", 3))
    b.add_track(Track("elephant", "tower", 4))
    b.add_mission(ConnectCitiesMission("baker", "tower",11))
    b.add_mission(ConnectCitiesMission("kingscross", "tower",7))
    b.add_mission(ConnectCitiesMission("british", "stpauls",4))
    b.add_mission(ConnectCitiesMission("baker", "trafalgar",5))
    b.add_mission(ConnectCitiesMission("trafalgar", "stpauls",4))
    b.add_mission(ConnectCitiesMission("buckingham", "elephant",5))
    b.add_mission(ConnectCitiesMission("bricklane", "buckingham",9))
    b.add_mission(ConnectCitiesMission("picadilly", "waterloo",3))
    b.add_mission(ConnectCitiesMission("covent", "tower",6))
    b.add_mission(ConnectCitiesMission("bigben", "tower",6))
    b.add_mission(ConnectCitiesMission("hyde", "stpauls",6))
    b.add_mission(ConnectCitiesMission("charterhouse", "bigben",5))
    b.add_mission(ConnectCitiesMission("trafalgar", "globe",4))
    b.add_mission(ConnectCitiesMission("bricklane", "globe",4))
    b.add_mission(ConnectCitiesMission("covent", "hyde",3))
    b.add_mission(ConnectCitiesMission("british", "waterloo",4))
    b.add_mission(ConnectCitiesMission("regent", "elephant",9))
    b.add_mission(ConnectCitiesMission("regent", "picadilly",5))
    b.add_mission(ConnectCitiesMission("british", "picadilly",2))
    b.add_mission(ConnectCitiesMission("kingscross", "buckingham",6))
    b.add_mission(ConnectDistrictsMission(["hyde", "baker", "regent", "kingscross"],5))
    b.add_mission(ConnectDistrictsMission(["charterhouse", "bricklane", "tower", "stpauls"],4))
    b.add_mission(ConnectDistrictsMission(["waterloo", "globe", "elephant"],3))
    b.add_mission(ConnectDistrictsMission(["bigben", "buckingham", "trafalgar", "picadilly"],2))
    b.add_mission(ConnectDistrictsMission(["british","covent"],1))
    return b

