package y2019.d11

import IntCodeComputer
import Vector

enum class Direction(val id: Int) {
    LEFT(3),
    RIGHT(1),
    UP(0),
    DOWN(2);

    companion object {
        private val map = values().associateBy(Direction::id)
        fun fromInt(value: Int): Direction {
            if (value == -1) return LEFT
            return map[value] ?: UP
        }
    }
}

enum class Colour(val id: Int) {
    BLACK(0),
    WHITE(1);

    companion object {
        private val map = values().associateBy(Colour::id)
        fun fromInt(type: Int) = map[type] ?: BLACK
    }
}

class PaintingRobot(instructions: String) : IntCodeComputer(instructions) {
    var direction = Direction.UP
    var position = Vector(0, 0)
    var canvas: MutableMap<Vector<Int>, Colour> = mutableMapOf()

    var acceptDirection: Boolean = false

    fun paint(): Map<Vector<Int>, Colour> {
        super.run()
        return canvas
    }

    fun paintFrom(startingPanel: Colour = Colour.WHITE): Map<Vector<Int>, Colour> {
        canvas[position] = startingPanel
        super.run()
        return canvas
    }

    override fun onInput(): Long {
        return canvas[position]?.id?.toLong() ?: 0
    }

    override fun onOutput(output: Long) {
        if (acceptDirection) {
            turnRightOtherwiseLeftAndMoveOneStepForward(output == 1.toLong())
            acceptDirection = false
            return
        }

        paint(Colour.fromInt(output.toInt()))
        acceptDirection = true
    }

    private fun turnRightOtherwiseLeftAndMoveOneStepForward(turnRight: Boolean) {
        val value = direction.id + (if (turnRight) 1 else -1)
        direction = Direction.fromInt(value)

        position = when (direction) {
            Direction.LEFT -> Vector(position.x - 1, position.y)
            Direction.RIGHT -> Vector(position.x + 1, position.y)
            Direction.UP -> Vector(position.x, position.y - 1)
            Direction.DOWN -> Vector(position.x, position.y + 1)
        }
    }

    private fun paint(c: Colour) {
        canvas[position] = c
    }

    fun print() {
        if (canvas.isEmpty()) return

        val minX = canvas.minBy { it.key.x }!!.key.x
        val maxX = canvas.maxBy { it.key.x }!!.key.x
        val minY = canvas.minBy { it.key.y }!!.key.y
        val maxY = canvas.maxBy { it.key.y }!!.key.y

        (minY..maxY).flatMap { y -> (minX until maxX).map { x -> canvas[Vector(x, y)] } }
            .map { colour ->
                when (colour) {
                    Colour.BLACK -> "░░"
                    Colour.WHITE -> "██"
                    else -> "░░"
                }
            }
            .chunked(maxX - minX)
            .map { it.reduce { acc, s -> acc + s } }
            .forEach { println(it) }
    }
}
