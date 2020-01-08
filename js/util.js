export function leadZeros(n,lz) {
    let result = n + "";
    while (result.length < lz) {
        result = "0" + result;
    }
    return result;
}

export function array_to_string(e) {
    return `${leadZeros(e[1],2)}:${leadZeros(e[2],2)}`;
}
