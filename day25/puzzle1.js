var column = 3019, row = 3010;

var sc = 20151125;
for (var y = 1;; ++y) {
    var yn = y;
    for (var x = 1; x <= y; ++x, --yn) {
        if (column == x && row == yn) {
            console.log("(%d,%d) = %d", x, yn, sc);
            return;
        }

        sc = (sc * 252533) % 33554393;
    }
}