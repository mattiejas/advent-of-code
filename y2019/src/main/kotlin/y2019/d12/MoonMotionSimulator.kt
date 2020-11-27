package y2019.d12

import Vector3
import java.security.MessageDigest
import javax.xml.bind.DatatypeConverter
import kotlin.math.abs


data class Moon(
    var id: Int = 0,
    var position: Vector3<Double>,
    var velocity: Vector3<Double> = Vector3(0.0, 0.0, 0.0)
) {
    override fun equals(other: Any?): Boolean {
        return other is Moon && other.id == id
    }

    fun hash(): String {
        val str = this.id.toString() + this.position.toString() + this.velocity.toString()

        val md = MessageDigest.getInstance("MD5")
        md.update(str.toByteArray())
        val digest = md.digest()
        return DatatypeConverter.printHexBinary(digest).toUpperCase()

//        val md = MessageDigest.getInstance("SHA-256")
//        val digest = md.digest(str.toByteArray())
//        return digest.fold("", { str, it -> str + "%02x".format(it) })
    }

    override fun hashCode(): Int {
        var result = id
        result = 31 * result + position.hashCode()
        result = 31 * result + velocity.hashCode()
        return result
    }
}

class MoonMotionSimulator(moons: List<Moon>) {
    private var _moons = moons
    private var _energy = mutableMapOf<Int, Int>()
    private var _history = mutableListOf<String>()
    private var time: Long = 0

    private fun step(): String {
        time++

        updateGravity()
        updatePositions()
        updateEnergy()

        return hashState()
    }

    private fun hashState() = _moons.map { it.hash() }.reduce { acc, s -> "$acc.$s" }

    fun simulate(n: Int): Int {
        for (i in 1..n) {
            step()
        }
        return _energy.map { it.value }.sum()
    }

    private fun areMoonsEqualOnAxis(moon1: Moon, moon2: Moon, axis: Int) =
        moon1.position[axis] == moon2.position[axis] && moon1.velocity[axis] == moon2.velocity[axis]


    private fun areMoonsEqualOnAxis(moons1: List<Moon>, moons2: List<Moon>, axis: Int): Boolean {
        return moons1.withIndex().map {
            areMoonsEqualOnAxis(
                it.value,
                moons2[it.index],
                axis
            )
        }.filter { it }.size == moons1.size
    }

    fun simulateUntilSame(): Long {
        val initial = _moons.map { it.copy() }
        var x: Long = 0
        var y: Long = 0
        var z: Long = 0

        while (x == 0L || y == 0L || z == 0L) {
            step()

            if (x == 0L && areMoonsEqualOnAxis(initial, _moons, 0)) x = time
            if (y == 0L && areMoonsEqualOnAxis(initial, _moons, 1)) y = time
            if (z == 0L && areMoonsEqualOnAxis(initial, _moons, 2)) z = time
        }

        return Utils.lcm(x, y, z)
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
