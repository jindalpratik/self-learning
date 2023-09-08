import mokkari
mokkari_sqlite_cache = mokkari.sqlite_cache.SqliteCache(db_name="mokkari_cache.db",expire=1)


from simyan.comicvine import Comicvine
from simyan.sqlite_cache import SQLiteCache as comicvine_sqlite_cache

from config import metron_username, metron_password, comicvine_api_key 

print("Mokkari verison :", mokkari.__version__)
m = mokkari.api(username = metron_username, passwd= metron_password, cache=mokkari_sqlite_cache)

def get_metron_id(cv_issue_id):
    metron_issue_list = m.issues_list({"cv_id" : cv_issue_id})

# Print the issue and get the series cv_id
    for metron_issue in metron_issue_list:
        print(metron_issue.id, metron_issue.series.name)
        metron_issue_details = m.issue(metron_issue.id)
        print(metron_issue_details.desc) # type: ignore
        metron_series_details = m.series(metron_issue_details.series.id) #type: ignore
        print(metron_series_details.cv_id) #type: ignore
        
get_metron_id(957621)


# Retrieve the detail for an individual issue
