open class IntCodeComputer(tokens: String) {
    class Operation(val code: Int, val modes: Array<Int>)

    private var ops: MutableMap<Long, Long>
    private var _position: Long = 0
    private var _paused = false
    private var _base: Long = 0

    init {
        ops = parseOps(tokens).toMutableMap()
    }

    private fun parseOps(str: String): Map<Long, Long> {
        return str.split(',')
            .mapIndexed { i, it -> i.toLong() to it.toLong() }
            .toMap()
    }

    private fun get(ops: Map<Long, Long>, position: Long): Operation {
        val op = ops[position].toString().padStart(5, '0')
        return Operation(
            op.substring(3, 5).toInt(),
            arrayOf(
                Integer.parseInt(op[2].toString()),
                Integer.parseInt(op[1].toString()),
                Integer.parseInt(op[0].toString())
            )
        )
    }

    private fun getValue(position: Long, mode: Int): Long {
        return when (mode) {
            0 -> ops.getOrPut(ops.getOrPut(position, { 0 }), { 0 }) // read value from absolute
            1 -> ops.getOrPut(position, { 0 }) // read as value
            2 -> ops.getOrPut(_base + ops.getOrPut(position, { 0 }), { 0 }) // read value from relative
            else -> error("UNKNOWN MODE $mode")
        }
    }

    private fun getPosition(position: Long, mode: Int): Long {
        return when (mode) {
            0 -> ops.getOrPut(position, { 0 }) // read value from absolute
            2 -> _base + ops.getOrPut(position, { 0 }) // read value from relative
            else -> error("CANNOT READ MODE $mode AS POSITION")
        }
    }

    open fun onOutput(output: Long) {
        println("OUTPUT: $output")
    }

    open fun onInput(): Long {
        print("> ")
        val input = readLine()!!
        return input.toLong()
    }

    open fun onHalt() {}

    private fun handleNext(): Boolean {
        val op = get(ops, _position)
        when (op.code) {
            1 -> {
                val a = getValue(_position + 1, op.modes[0])
                val b = getValue(_position + 2, op.modes[1])
                ops[getPosition(_position + 3, op.modes[2])] = a + b
                _position += 4
                return true
            }
            2 -> {
                val a = getValue(_position + 1, op.modes[0])
                val b = getValue(_position + 2, op.modes[1])
                ops[getPosition(_position + 3, op.modes[2])] = a * b
                _position += 4
                return true
            }
            3 -> {
                val pos = getPosition(_position + 1, op.modes[0])
                ops[pos] = onInput()
                _position += 2
                return true
            }
            4 -> {
                val output = getValue(_position + 1, op.modes[0])
                onOutput(output)
                _position += 2
                return true
            }
            5 -> {
                val a = getValue(_position + 1, op.modes[0])
                val b = getValue(_position + 2, op.modes[1])
                if (a != 0.toLong()) _position = b else _position += 3
                return true
            }
            6 -> {
                val a = getValue(_position + 1, op.modes[0])
                val b = getValue(_position + 2, op.modes[1])
                if (a == 0.toLong()) _position = b else _position += 3
                return true
            }
            7 -> {
                val a = getValue(_position + 1, op.modes[0])
                val b = getValue(_position + 2, op.modes[1])
                ops[getPosition(_position + 3, op.modes[2])] = (if (a < b) 1 else 0).toLong()
                _position += 4
                return true
            }
            8 -> {
                val a = getValue(_position + 1, op.modes[0])
                val b = getValue(_position + 2, op.modes[1])
                ops[getPosition(_position + 3, op.modes[2])] = (if (a == b) 1 else 0).toLong()
                _position += 4
                return true
            }
            9 -> {
                val a = getValue(_position + 1, op.modes[0])
                _base += a
                _position += 2
                return true
            }
            99 -> {
                onHalt()
                return false
            }
            else -> error("OP_CODE = ${op.code} NOT RECOGNIZED")
        }
    }

    open fun run() {
        _paused = false
        while (!_paused && this.handleNext());
    }

    fun pause() {
        _paused = true
    }
}
