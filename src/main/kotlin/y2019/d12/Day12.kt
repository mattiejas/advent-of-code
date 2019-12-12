package y2019.d12

import Day
import Vector3

class Day12 : Day {
    private val moons = Utils.readFile("2019/12/input")
        .map { it.trim { c -> c == '<' || c == '>' } }
        .map { it.split(',').map { it.trim().split('=')[1].toDouble() } }
        .withIndex()
        .map { Moon(it.index + 1, Vector3(it.value[0], it.value[1], it.value[2])) }

    override fun partOne() {
        println(moons)
        val energy = MoonMotionSimulator(moons).simulate(1000)
        println(energy)
    }

    override fun partTwo() {
        println(MoonMotionSimulator(moons).simulateUntilSame())
    }
}
