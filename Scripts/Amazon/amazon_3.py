from urllib.request import urlopen
from bs4 import BeautifulSoup
import ssl
import re
import html5lib

# Ignore SSL certificate errors
ctx = ssl.create_default_context()
ctx.check_hostname = False
ctx.verify_mode = ssl.CERT_NONE

fname = input('Enter a file name: ').strip('"')
try:
    fhand = open(fname)
except:
    print('Not a correct file name.')
    exit()

# url = input('Enter - ')
# html = urlopen(url, context=ctx).read()
soup = BeautifulSoup(fhand, "html5lib")
# Retrieve all of the anchor tags
tags = soup('li')
temp_pos = 0
for tag in tags:
    tag_class = tag.get('data-price')
    try:
        print(tag_class)
    except:
        continue
    # Name = re.findall('known_by_(.*).html' , url)
