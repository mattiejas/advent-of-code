package y2019.d12

import Vector3
import java.security.MessageDigest
import kotlin.math.abs


class Moon(
    var id: Int = 0,
    var position: Vector3<Double>,
    var velocity: Vector3<Double> = Vector3(0.0, 0.0, 0.0)
) {
    override fun equals(other: Any?): Boolean {
        return other is Moon && other.id == id
    }

    fun hash(): String {
        val str = this.id.toString() +this.position.toString() + this.velocity.toString()
        val md = MessageDigest.getInstance("SHA-256")
        val digest = md.digest(str.toByteArray())
        return digest.fold("", { str, it -> str + "%02x".format(it) })
    }
}

class MoonMotionSimulator(moons: List<Moon>) {
    private var _moons = moons
    private var _energy = mutableMapOf<Int, Int>()
    private var _history = mutableListOf<String>()
    private var time: Long = 0

    private fun step(): String {
        updateGravity()
        updatePositions()
        updateEnergy()

        time++
        return hashState()
    }

    private fun hashState() = _moons.map { it.hash() }.reduce { acc, s -> "$acc.$s" }

    fun simulate(n: Int): Int {
        for (i in 1..n) {
            step()
        }
        return _energy.map { it.value }.sum()
    }

    fun simulateUntilSame(): Long {
        var hash: String

        do {
            if (time == 20891.toLong()) {
                val pause = true
            }
            hash = step()
            _history.add(hash)
        } while (_history.count { it == hash } == 1)

        val lastOccurence = _history.withIndex().find { it.value == hash }!!.index
        return time
    }

    private fun updateEnergy() {
        _moons.forEach {
            val pot = abs(it.position.x) + abs(it.position.y) + abs(it.position.z)
            val kin = abs(it.velocity.x) + abs(it.velocity.y) + abs(it.velocity.z)
            _energy[it.id] = (pot * kin).toInt()
        }
    }

    private fun updatePositions() {
        _moons = _moons.map { it.position += it.velocity; return@map it }
    }

    private fun updateGravity() {
        val delta = _moons.map { it to Pair(_moons.filterNot { m -> m == it }, Vector3(0, 0, 0)) }
            .map {
                val moon = it.first
                val list = it.second.first
                list.map { other ->
                    val gX = other.position.x - moon.position.x
                    if (gX > 0) it.second.second.x += 1
                    if (gX < 0) it.second.second.x -= 1

                    val gY = other.position.y - moon.position.y
                    if (gY > 0) it.second.second.y += 1
                    if (gY < 0) it.second.second.y -= 1

                    val gZ = other.position.z - moon.position.z
                    if (gZ > 0) it.second.second.z += 1
                    if (gZ < 0) it.second.second.z -= 1
                }
                return@map it
            }
            .map { it.first to it.second.second }.toMap()

        _moons = _moons.map { it.velocity += delta[it]!!.castTo(); return@map it }
    }
}
