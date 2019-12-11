class AmpArray(ops: String, settings: List<Int>) {
    private var _index = -1
    private var _count = 0
    var output = 0

    private val amps = listOf(
        AmpArrayComputer(ops, settings[_count++], this),
        AmpArrayComputer(ops, settings[_count++], this),
        AmpArrayComputer(ops, settings[_count++], this),
        AmpArrayComputer(ops, settings[_count++], this),
        AmpArrayComputer(ops, settings[_count++], this)
    )

    private fun runNextAmp() {
        if (_index != -1) {
            output = amps[_index].output
        }
        _index++

        if (_index == _count) _index = 0
        amps[_index].run()
    }

    fun run(): Int {
        while (!amps[_count - 1].halted) {
            runNextAmp()
        }
        output = amps[_count - 1].output
        return output
    }
}

class AmpArrayComputer(ops: String, private val phase: Int, private val array: AmpArray) :
    IntCodeComputer(ops) {
    var output = -1
    var initialized = false
    var halted = false

    override fun onOutput(output: Long) {
        this.output = output.toInt()
        this.pause()
    }

    override fun onInput(): Long {
        if (!initialized) {
            initialized = true
            return phase.toLong()
        }
        return array.output.toLong()
    }

    override fun onHalt() {
        halted = true
    }

    override fun run() {
        if (!halted) return super.run()
    }
}
