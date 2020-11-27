package y2019.d10

import Vector

class AsteroidField(data: List<String>) {
    private var _map = mutableListOf<List<Boolean>>()
    private var _amountInSight = mapOf<Vector<Int>, Int>()
    private val _best: Vector<Int>

    init {
        _map = data.map { it.map { a -> a == '#' } }.toMutableList()
        _best = findBestWatchtowerSpot()
    }

    fun amount(x: Int, y: Int): Int? {
        return _amountInSight[Vector(x, y)]
    }

    fun watchtower(): Vector<Int> {
        return _best
    }

    private fun asteroids(): List<Vector<Int>> =
        _map.indices.flatMap { y -> _map[y].indices.map { x -> Vector(x, y) to _map[y][x] } }
            .filter { p -> p.second }.map { it.first }

    private fun findBestWatchtowerSpot(): Vector<Int> {
        _amountInSight = asteroids()
            .map { it to getAmountInSight(it) }
            .toMap()

        return _amountInSight.maxBy { it.value }!!.key
    }

    private fun getAmountInSight(p: Vector<Int>): Int {
        val amount = asteroids().filter { it != p }
            .map { it.angle(p) }
            .distinct()

        return amount.size
    }

    fun vaporize(): List<Vector<Int>> {
        val angles = asteroids().groupBy { watchtower().angle(it) }
            .map { it.key to it.value.sortedBy { it.magnitude() }.toMutableList() }
            .sortedByDescending { it.first }
            .toMutableList()

        val vaporized = mutableListOf<Vector<Int>>()
        var index = 0
        while (angles.isNotEmpty()) {
            val list = angles[index].second

            if (list.isNotEmpty()) {
                vaporized.add(list.removeAt(0))

                if (list.isEmpty()) { // remove if it is empty
                    angles.removeAt(index)
                } else {
                    index++
                }

                if (index >= angles.size) index = 0
            }
        }
        return vaporized
    }
}
