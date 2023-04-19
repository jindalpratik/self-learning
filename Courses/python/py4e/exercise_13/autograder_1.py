import xml.etree.ElementTree as ET
from urllib.request import urlopen
import ssl

# Ignore SSL certificate errors
ctx = ssl.create_default_context()
ctx.check_hostname = False
ctx.verify_mode = ssl.CERT_NONE

url = input('Enter location: ')
print("Retrieving",url)
xml = urlopen(url, context=ctx).read().decode()

# print(xml)

tree = ET.fromstring(xml)
counts = tree.findall('comments/comment')
print("count =",len(counts))

sum = 0

for count in counts:
    cur_count = count.find('count')
    if cur_count is not None:
        cur_count_text = cur_count.text
    else:
        continue
    
    assert cur_count_text is not None
    cur_count_num = int(cur_count_text)
    sum += cur_count_num
    
print("Sum", sum)