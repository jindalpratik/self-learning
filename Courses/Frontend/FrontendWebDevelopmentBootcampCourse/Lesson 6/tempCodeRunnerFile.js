const counts = {};
const sampleArray = ['a', 'a', 'b', 'c'];
sampleArray.forEach(function (x) { counts[x] = (counts[x] || 0) + 1; });
console.log(counts)
let newcount = 0;
for(let newele in counts) {
    if(counts[newele] > 1){ newcount += 1;}
}
console.log(newcount)