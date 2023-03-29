text = "X-DSPAM-Confidence:    0.8475"
x_text = text.strip()
colpos = x_text.find(':')
#print(colpos)
f_text = float(x_text[colpos + 1 :])
print(f_text)