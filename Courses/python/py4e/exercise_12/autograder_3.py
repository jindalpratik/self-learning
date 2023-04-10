from urllib.request import urlopen
from bs4 import BeautifulSoup
import ssl
import re

# Ignore SSL certificate errors
ctx = ssl.create_default_context()
ctx.check_hostname = False
ctx.verify_mode = ssl.CERT_NONE

count = int(input("Enter count: "))
position = int(input("Enter position: "))
url = input('Enter - ')
Name = re.findall('known_by_(.*).html' , url)
print(Name[0], end = " ")
for i in range(count):
    html = urlopen(url, context=ctx).read()
    soup = BeautifulSoup(html, "html.parser")
    # Retrieve all of the anchor tags
    tags = soup('a')
    temp_pos = 0
    for tag in tags:
        temp_pos += 1
        if temp_pos == position:
            url = tag.get('href', None)
            Name = re.findall('known_by_(.*).html' , url)
            print(Name[0], end = " ")
            # print(url)



