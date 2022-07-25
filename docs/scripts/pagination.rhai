let arr = params[0];
let index = params[1];
let offset = params[2];

let val = #{
    prev: "",
    next: "",
    subarr: []
};

val.subarr = arr.extract(index * offset, offset);

if (arr.len > (index*offset) + offset ) {
    val.next = index + 1;
}

if index > 0 {
    if (arr.len < index*offset) {
        val.prev = arr.len/offset;
    }
    else {
        val.prev = index - 1;
    }
}

val
