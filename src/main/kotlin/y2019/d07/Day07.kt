class Day07 : Day {
    val input = Utils.readFile("2019/07/input")[0]

    private fun findMaxOutput(settingsRange: IntRange = 0..4): Int {
        val outputs = mutableListOf<Int>()
        for (i in settingsRange) {
            for (j in settingsRange) {
                for (k in settingsRange) {
                    for (l in settingsRange) {
                        for (m in settingsRange) {
                            if (listOf(i, j, k, l, m).distinct().size != 5) continue
                            val array = AmpArray(input, listOf(i, j, k, l, m))
                            outputs.add(array.run())
                        }
                    }
                }
            }
        }
        return outputs.max()!!
    }

    override fun partOne() {
        println(findMaxOutput(0..4))
    }

    override fun partTwo() {
        println(findMaxOutput(5..9))
    }
}
