import json
from urllib.request import urlopen
import ssl

# Ignore SSL certificate errors
ctx = ssl.create_default_context()
ctx.check_hostname = False
ctx.verify_mode = ssl.CERT_NONE

url = input('Enter location: ')
print("Retrieving",url)
raw_json = urlopen(url, context=ctx).read().decode()

# print(raw_json)

parsed_json = json.loads(raw_json)

sum = 0

comments = parsed_json['comments']
print("count:", len(comments))

for item in comments:
    count = item['count']
    sum += count
    
print("sum:",sum)