import kotlinx.coroutines.*

class AmpArray(ops: List<Int>, settings: List<Int>) {
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
        while(!amps[_count - 1].halted) {
            runNextAmp()
        }
        output = amps[_count - 1].output
        return output
    }
}

class AmpArrayComputer(ops: List<Int>, private val phase: Int, private val array: AmpArray) :
    IntCodeComputer(ops.toMutableList()) {
    var output = -1
    var initialized = false
    var halted = false

    override fun onOutput(output: Int) {
        this.output = output
        this.pause()
    }

    override fun onInput(): Int {
        if (!initialized) {
            initialized = true
            return phase
        }
        return array.output
    }

    override fun onHalt() {
        halted = true
    }

    override fun run() {
        if (!halted) return super.run()
    }
}
