class Day08 : Day {
    private val data = Utils.readFile("2019/08/input")[0]
        .toCharArray()
        .map { Integer.parseInt(it.toString()) }

    class Image(private val width: Int, height: Int, data: List<Int>) {
        var layers: MutableList<List<Int>> = mutableListOf()
        var amountOfLayers: Int = 0

        init {
            while ((amountOfLayers + 1) * (width * height) <= data.size) {
                layers.add(data.subList(amountOfLayers * (width * height), ++amountOfLayers * (width * height)))
            }
        }

        private fun decode(): List<Int> {
            return layers.reduce { acc, next -> acc.mapIndexed { index, color -> if (color == 2) next[index] else color } }
        }

        fun display() {
            decode()
                .map { if (it == 1) "██" else "░░" }
                .chunked(width)
                .map { it.reduce { acc, s -> acc + s } }
                .forEach { color -> println(color) }
        }
    }

    override fun partOne() {
        val i = Image(25, 6, data)

        val layer = i.layers.minBy { it.count { it == 0 } }
        val one = layer?.count { it == 1 }
        val two = layer?.count { it == 2 }
        println(one!! * two!!)
    }

    override fun partTwo() {
        val i = Image(25, 6, data)

        i.display()
    }
}
