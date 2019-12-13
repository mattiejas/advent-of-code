package y2019.d13

import IntCodeComputer
import Vector
import y2019.d11.Colour

class Arcade(game: String) : IntCodeComputer(game) {
    private var _score = 0
    private var _steps = 0
    private var _position = Vector(0,0)
    private var _tileId = 0

    private var _output = mutableListOf<Any>()
    private var _canvas = mutableMapOf<Vector<Int>, Int>()

    override fun onOutput(output: Long) {
        when {
            _steps % 3 == 0 -> _position = Vector(output.toInt(), 0)
            _steps % 3 == 1 -> _position.y = output.toInt()
            _steps % 3 == 2 -> {
                _tileId = output.toInt()
                render(_position to _tileId)
            }
        }
        _steps++

//        if (_output.size != 3) {
//            _output.add(output.toInt())
//            return
//        }
//
//        val pair = parseOutput(_output)
//        if (pair.first == Vector(-1, 0)) {
//            _score = pair.second
//            return
//        }
//
//        render(pair)
    }

    override fun onInput(): Long {
        print()
        return 0
    }

    override fun onHalt() {
        println(_canvas.filter { it.value == 2 }.size)
    }

    private fun parseOutput(output: List<Any>): Pair<Vector<Int>, Int> {
        val position = Vector(output[0] as Int, output[1] as Int)
        val tileId = output[2] as Int
        return position to tileId
    }

    private fun render(tile: Pair<Vector<Int>, Int>) {
        _canvas[tile.first] = tile.second
        _output = mutableListOf()
    }

    private fun print() {
        val minX = _canvas.minBy { it.key.x }!!.key.x
        val maxX = _canvas.maxBy { it.key.x }!!.key.x
        val minY = _canvas.minBy { it.key.y }!!.key.y
        val maxY = _canvas.maxBy { it.key.y }!!.key.y

        for (y in minY..maxY) {
            for (x in minX..maxX) {
                print(
                    when (_canvas[Vector(x, y)]) {
                        0 -> " "
                        1 -> "█"
                        2 -> "░"
                        3 -> "_"
                        4 -> "*"
                        else -> ""
                    }
                )
            }
            println()
        }
    }
}
