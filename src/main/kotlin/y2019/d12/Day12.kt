package y2019.d12

import Day
import Vector3

class Day12 : Day {
    fun get() = Utils.readFile("2019/12/input")
        .map { it.trim { c -> c == '<' || c == '>' } }
        .map { it.split(',').map { it.trim().split('=')[1].toDouble() } }
        .withIndex()
        .map { Moon(it.index + 1, Vector3(it.value[0], it.value[1], it.value[2])) }


    override fun partOne() {
        println(get())
        val energy = MoonMotionSimulator(get()).simulate(1000)
        println(energy)
    }

    override fun partTwo() {
        val one = get().map { it.hash() }.reduce { acc, s -> "$acc.$s" }

        val two = Utils.readFile("2019/12/input")
            .map { it.trim { c -> c == '<' || c == '>' } }
            .map { it.split(',').map { it.trim().split('=')[1].toDouble() } }
            .withIndex()
            .map { Moon(it.index + 1, Vector3(it.value[0], it.value[1], it.value[2])) }.map { it.hash() }.reduce { acc, s -> "$acc.$s" }

        println(one == two)
        println(MoonMotionSimulator(get()).simulateUntilSame())
    }
}
