package y2019.d11

import Day
import Utils

class Day11 : Day {
    val input = Utils.readFile("2019/11/input")[0]

    override fun partOne() {
        PaintingRobot(input).apply {
            println(paint().size)
            print()
        }
    }

    override fun partTwo() {
        PaintingRobot(input).apply {
            paintFrom(Colour.WHITE)
            print()
        }
    }
}
