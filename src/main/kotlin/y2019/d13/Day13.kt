package y2019.d13

import Day

class Day13 : Day {
    val input = Utils.readFile("2019/13/input")[0]

    override fun partOne() {
        Arcade(input).run()
    }

    @ExperimentalStdlibApi
    override fun partTwo() {
        val input2 = input.toCharArray()
        input2[0] = '2'

        Arcade(input2.concatToString()).apply {
            run()
        }
    }
}
