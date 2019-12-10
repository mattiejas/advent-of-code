import java.io.File

val input = File("input").readLines()
    .map { l -> l.split(',') }
    .flatten()
    .map { i -> i.toInt() }

fun main() {
    println(runAmps())
}

fun runAmps(): Int {
    val outputs = mutableListOf<Int>()
    for (i in 0..4) {
        for (j in 0..4) {
            for (k in 0..4) {
                for (l in 0..4) {
                    for (m in 0..4) {
                        if (listOf(i, j, k, l, m).distinct().size != 5) continue
                        val output = runAmp(listOf(i, j, k, l, m))
                        outputs.add(output)
                    }
                }
            }
        }
    }
    return outputs.max()!!
}

fun runAmp(settings: List<Int>): Int {
    var output = 0
    for (i in settings) {
        output = handleOp(input, 0, listOf(i, output))[0]
    }
    return output
}

class Operation(val code: Int, val mode1: Boolean, val mode2: Boolean, val mode3: Boolean)

fun get(ops: List<Int>, position: Int): Operation {
    val op = ops[position].toString().padStart(5, '0')
    return Operation(
        op.substring(3, 5).toInt(),
        op[2] != '0',
        op[1] != '0',
        op[0] != '0'
    )
}

fun getValue(ops: List<Int>, position: Int, mode: Boolean): Int {
    return if (mode) ops[position] else ops[ops[position]]
}

fun handleOp(
    operations: List<Int>,
    position: Int,
    inputs: List<Int> = emptyList(),
    outputs: List<Int> = emptyList()
): List<Int> {
    val ops = operations.toMutableList()
    val op = get(operations, position)
    when (op.code) {
        1 -> {
            val a = getValue(ops, position + 1, op.mode1)
            val b = getValue(ops, position + 2, op.mode2)
            ops[ops[position + 3]] = a + b
            return handleOp(ops, position + 4, inputs, outputs)
        }
        2 -> {
            val a = getValue(ops, position + 1, op.mode1)
            val b = getValue(ops, position + 2, op.mode2)
            ops[ops[position + 3]] = a * b
            return handleOp(ops, position + 4, inputs, outputs)
        }
        3 -> {
            if (inputs.isEmpty()) {
                print("Input variable\n> ")
                val input = readLine()!!
                ops[ops[position + 1]] = input.toInt()
            } else {
                ops[ops[position + 1]] = inputs.first()
            }
            return handleOp(ops, position + 2, inputs.drop(1), outputs)
        }
        4 -> {
            val output = getValue(ops, position + 1, op.mode1)
            println("Output: $output")
            return handleOp(ops, position + 2, inputs, outputs.plus(output))
        }
        5 -> {
            val a = getValue(ops, position + 1, op.mode1)
            val b = getValue(ops, position + 2, op.mode2)
            return handleOp(ops, if (a != b) b else position + 3, inputs, outputs)
        }
        6 -> {
            val a = getValue(ops, position + 1, op.mode1)
            val b = getValue(ops, position + 2, op.mode2)
            return handleOp(ops, if (a == b) b else position + 3, inputs, outputs)
        }
        7 -> {
            val a = getValue(ops, position + 1, op.mode1)
            val b = getValue(ops, position + 2, op.mode2)
            ops[ops[position + 3]] = if (a < b) 1 else 0
            return handleOp(ops, position + 4, inputs, outputs)
        }
        8 -> {
            val a = getValue(ops, position + 1, op.mode1)
            val b = getValue(ops, position + 2, op.mode2)
            ops[ops[position + 3]] = if (a == b) 1 else 0
            return handleOp(ops, position + 4, inputs, outputs)
        }
        99 -> return outputs
        else -> error("OP_CODE = ${op.code} NOT RECOGNIZED")
    }
}
