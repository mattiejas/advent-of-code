package y2019.d13

import IntCodeComputer
import Vector
import y2019.d11.Colour

class Arcade(game: String) : IntCodeComputer(game) {
    private var _score = 0
    private var _steps = 0
    private var _position = Vector(0,0)
    private var _tileId = 0
    private var _input = 0

    private var _output = mutableListOf<Any>()
    var canvas = mutableMapOf<Vector<Int>, Int>()

    override fun onOutput(output: Long) {
        when {
            _steps % 3 == 0 -> _position = Vector(output.toInt(), 0)
            _steps % 3 == 1 -> _position.y = output.toInt()
            _steps % 3 == 2 -> {
                if (_position == Vector(-1, 0)) _score = output.toInt()
                else {
                    _tileId = output.toInt()
                    render(_position to _tileId)
                }
            }
        }
        _steps++
    }

    override fun beforeInput() {
        this.pause()
    }

    override fun onInput(): Long {
        val i = _input
        _input = 0 // reset
        return i.toLong()
    }

    override fun onHalt() {
        println(canvas.filter { it.value == 2 }.size)
        println(_score)
    }

    private fun render(tile: Pair<Vector<Int>, Int>) {
        canvas[tile.first] = tile.second
    }

    fun dimensions() : Vector<Int> {
        if (canvas.size < 5) return Vector(0, 0)

        val maxX = canvas.maxBy { it.key.x }!!.key.x + 1
        val maxY = canvas.maxBy { it.key.y }!!.key.y + 1
        return Vector(maxX, maxY)
    }

    fun input(i: Int) {
        _input = i
    }

    fun ball(): Vector<Int> {
        return canvas.filter { it.value == 4 }.keys.first()
    }

    fun paddle(): Vector<Int> {
        return canvas.filter { it.value == 3 }.keys.first()
    }
}
