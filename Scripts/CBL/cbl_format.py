# imports related to mokkari
import mokkari

# imports related to simyan
import simyan
from simyan.comicvine import Comicvine
from simyan.sqlite_cache import SQLiteCache as comicvine_sqlite_cache

# secrets stored in config.py file
from config import metron_username, metron_password, comicvine_api_key 

# initializing simyan
print("Simyan verison : ", simyan.__version__)
c = Comicvine(api_key="Comicvine API Key", cache=comicvine_sqlite_cache(path = "comicvine_cache.db", expiry=1)) #type: ignore

# initializing mokkari
print("Mokkari verison :", mokkari.__version__)
mokkari_sqlite_cache = mokkari.sqlite_cache.SqliteCache(db_name="metron_cache.db",expire=1)
m = mokkari.api(username = metron_username, passwd= metron_password, cache=mokkari_sqlite_cache)

# Get metron series id and issue id from cv_id
def get_metron_id(cv_issue_id):
    metron_issue_list = m.issues_list({"cv_id" : cv_issue_id})

# Print the issue and get the series cv_id
    for metron_issue in metron_issue_list:
        print(metron_issue.id, metron_issue.series.name)
        metron_issue_details = m.issue(metron_issue.id)
        print(metron_issue_details.desc) # type: ignore
        # metron_series_details = m.series(metron_issue_details.series.id) #type: ignore
        # print(metron_series_details.cv_id) #type: ignore
        
get_metron_id(957621)
