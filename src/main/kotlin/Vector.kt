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
