import java.security.MessageDigest
import kotlin.math.atan2

class Vector<T : Number>(var x: T, var y: T) {
    fun angle(other: Vector<T>): Double {
        return atan2(other.x.toDouble() - this.x.toDouble(), other.y.toDouble() - this.y.toDouble())
    }

    fun minus(other: Vector<T>): Vector<Double> {
        return Vector((this.x.toDouble() - other.x.toDouble()), (this.y.toDouble() - other.y.toDouble()))
    }

    override fun toString(): String {
        return "[ x: $x, y: $y ]"
    }

    fun magnitude(): Double {
        return (x.toDouble() * x.toDouble()) + (y.toDouble() * y.toDouble())
    }

    fun normalize(): Vector<Double> {
        return Vector(x.toDouble() / magnitude(), y.toDouble() / magnitude())
    }

    override fun equals(other: Any?): Boolean {
        return other is Vector<*> && this.x == other.x && this.y == other.y
    }

    override fun hashCode(): Int {
        var result = x.hashCode()
        result = 31 * result + y.hashCode()
        return result
    }
}

class Vector3<T : Number>(var x: T, var y: T, var z: T) {

    override fun equals(other: Any?): Boolean {
        return other is Vector3<*>
                && this.x == other.x && this.y == other.y && this.z == other.z
    }

    operator fun minus(other: Vector3<T>): Vector3<T> {
        return Vector3(
            (this.x - other.x),
            (this.y - other.y),
            (this.z - other.z)
        ) as Vector3<T>
    }

    operator fun plus(other: Vector3<T>): Vector3<T> {
        return Vector3(
            (this.x + other.x),
            (this.y + other.y),
            (this.z + other.z)
        ) as Vector3<T>
    }

    fun <S : Number> castTo(): Vector3<S> {
        return Vector3<S>(
            this.x as S,
            this.y as S,
            this.z as S
        )
    }

    fun magnitude(): Double {
        return (x.toDouble() * x.toDouble()) + (y.toDouble() * y.toDouble()) + (z.toDouble() * z.toDouble())
    }

    override fun hashCode(): Int {
        var hash = 17
        hash = hash * 23 + x.hashCode()
        hash = hash * 23 + y.hashCode()
        hash = hash * 23 + z.hashCode()
        return hash
    }
}

operator fun Number.minus(other: Number): Number {
    return when (this) {
        is Long -> this.toLong() - other.toLong()
        is Int -> this.toInt() - other.toInt()
        is Short -> this.toShort() - other.toShort()
        is Byte -> this.toByte() - other.toByte()
        is Double -> this.toDouble() - other.toDouble()
        is Float -> this.toFloat() - other.toFloat()
        else -> throw RuntimeException("Unknown numeric type")
    }
}

operator fun Number.plus(other: Number): Number {
    return when (this) {
        is Long -> this.toLong() + other.toLong()
        is Int -> this.toInt() + other.toInt()
        is Short -> this.toShort() + other.toShort()
        is Byte -> this.toByte() + other.toByte()
        is Double -> this.toDouble() + other.toDouble()
        is Float -> this.toFloat() + other.toFloat()
        else -> throw RuntimeException("Unknown numeric type")
    }
}

