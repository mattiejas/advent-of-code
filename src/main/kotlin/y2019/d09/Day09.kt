class Day09 : Day {
    override fun partOne() {
        val tokens = Utils.readFile("2019/09/input")[0]
        val comp = IntCodeComputer(tokens)
        comp.run()
    }

    override fun partTwo() {
        val tokens = Utils.readFile("2019/09/input")[0]
        val comp = IntCodeComputer(tokens)
        comp.run()
    }
}
