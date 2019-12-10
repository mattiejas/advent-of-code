abstract class IntCodeComputer(private val ops: MutableList<Int>) {
    class Operation(val code: Int, val mode1: Boolean, val mode2: Boolean, val mode3: Boolean)

    private var _position: Int = 0
    private var _paused = false

    private fun get(ops: List<Int>, position: Int): Operation {
        val op = ops[position].toString().padStart(5, '0')
        return Operation(
            op.substring(3, 5).toInt(),
            op[2] != '0',
            op[1] != '0',
            op[0] != '0'
        )
    }

    private fun getValue(ops: List<Int>, position: Int, mode: Boolean): Int {
        return if (mode) ops[position] else ops[ops[position]]
    }

    abstract fun onOutput(output: Int)
    abstract fun onInput(): Int
    abstract fun onHalt()

    private fun handleNext(): Boolean {
        val op = get(ops, _position)
        when (op.code) {
            1 -> {
                val a = getValue(ops, _position + 1, op.mode1)
                val b = getValue(ops, _position + 2, op.mode2)
                ops[ops[_position + 3]] = a + b
                _position += 4
                return true
            }
            2 -> {
                val a = getValue(ops, _position + 1, op.mode1)
                val b = getValue(ops, _position + 2, op.mode2)
                ops[ops[_position + 3]] = a * b
                _position += 4
                return true
            }
            3 -> {
                ops[ops[_position + 1]] = onInput()
                _position += 2
                return true
            }
            4 -> {
                val output = getValue(ops, _position + 1, op.mode1)
                onOutput(output)
                _position += 2
                return true
            }
            5 -> {
                val a = getValue(ops, _position + 1, op.mode1)
                val b = getValue(ops, _position + 2, op.mode2)
                if (a != b) _position = b else _position += 3
                return true
            }
            6 -> {
                val a = getValue(ops, _position + 1, op.mode1)
                val b = getValue(ops, _position + 2, op.mode2)
                if (a == b) _position = b else _position += 3
                return true
            }
            7 -> {
                val a = getValue(ops, _position + 1, op.mode1)
                val b = getValue(ops, _position + 2, op.mode2)
                ops[ops[_position + 3]] = if (a < b) 1 else 0
                _position += 4
                return true
            }
            8 -> {
                val a = getValue(ops, _position + 1, op.mode1)
                val b = getValue(ops, _position + 2, op.mode2)
                ops[ops[_position + 3]] = if (a == b) 1 else 0
                _position += 4
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
