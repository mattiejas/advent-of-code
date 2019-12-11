package y2019.d10

import Day
import Utils

class Day10 : Day {
    private val input = Utils.readFile("2019/10/input")
    private val field = AsteroidField(input)

    override fun partOne() {
        println(
            "${field.watchtower()}, amount: ${field.amount(
                field.watchtower().x,
                field.watchtower().y
            )}"
        )
    }

    override fun partTwo() {
        val a200 = field.vaporize()[199]
        println(a200)
    }
}
