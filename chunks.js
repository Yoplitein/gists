const chunks = (
    (array, size) =>
        Array.from(
            Array(array.length / size),
            (_, x) =>
                Array.from(
                    Array(size),
                    (_, y) => array[size * x + y]
                )
        )
);