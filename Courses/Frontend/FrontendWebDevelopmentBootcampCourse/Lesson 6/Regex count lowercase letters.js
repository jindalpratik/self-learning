function lowercaseCount(str){
    const arrstr = str.split("");
    const sum = arrstr.reduce((count , element) => {
        if(/[a-z]/.test(element)){return count + 1;}
        else{return count;}
    }, 0)
    return sum;
}