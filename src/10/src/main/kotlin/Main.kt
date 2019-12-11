import kotlin.math.atan
import kotlin.math.atan2

val data = "......#.#.#..#.#......#######..#.#.###...#..#.......#....#.##..#....#..##.#..#####...#..#..#....####"
val data2 = "#.#...#.#..###....#..#....#...##.#.#.#.#....#.#.#..##..###.#..#...##....##....##......#....####.###."
val data3 = ".#..#..#######.###.#....###.#...###.##.###.##.#.#.....###..#..#.#..#.##..#.#.###.##...##.#.....#.#.."
val data4 =
    ".#..##.###...#########.############..##..#.######.########.#.###.#######.####.#.#####.##.#.##.###.##..#####..#.##############################.####....###.#.#.####.######################.##.###..####....######..##.###########.##.####...##..#.#####..#.######.#####...#.##########...#.##########.#######.####.#.###.###.#.##....##.##.###..#####.#.#.###########.####.#.#.#####.####.######.##.####.##.#..##"

val input =
    ".###.#...#.#.##.#.####...#....#####...#.######..#.#.###.###.#.....#.######.###..##..####.#.####.###########.#######.##.###########.#########.##..#.##.########.##...###.###.#.##.#####.#.###.#####.#####.##..###.#.##.#..#.#.#####.####.#..#####.###.#####.#..#..##.#.##########.##.#...########.####..##..#.###.###.#.#....######.##.#.######.####.####.######.#....###############.#.#.##.######...##..####.####.#..##.###.#########.###..#.###.##.#.#...##...#####..###.#..###############.####.###.#####.##.######..##.#####.#.#.##..#######...#######.######...#####....#.#.#.####.#.#.#.##"


val data5 = "#...#..#..#...#"
fun main() {
    val a = AstroidField(input, 24, 24)
    a.print()
    println("${a.watchtower()}, amount: ${a.amount(a.watchtower().x.toInt(), a.watchtower().y.toInt())}")
    a.printResult()
}

class AstroidField(val data: String, val width: Int, val height: Int) {
    private val _map = mutableListOf<List<Boolean>>()
    private var _amountInSight = mapOf<Vector<Int>, Int>()
    private val _best: Vector<Int>

    init {
        val size = width
        var count = 0
        while ((count + 1) * size <= data.length) {
            _map += data.substring(count * size, ++count * size)
                .toCharArray()
                .map { it == '#' }
        }

        _best = findBestWatchtowerSpot()
    }

    private fun hasAstroidAt(x: Int, y: Int): Boolean {
        return _map[y][x]
    }

    fun amount(x: Int, y: Int): Int? {
        return _amountInSight[Vector(x, y)]
    }

    fun watchtower(): Vector<Int> {
        return _best
    }

    private fun asteroids(): List<Vector<Int>> =
        _map.indices.flatMap { y -> _map[y].indices.map { x -> Vector(x, y) to _map[y][x] } }
            .filter { p -> p.second }.map { it.first }

    private fun findBestWatchtowerSpot(): Vector<Int> {
        _amountInSight = asteroids()
            .map { it to getAmountInSight(it) }
            .toMap()

        return _amountInSight.maxBy { it.value }!!.key
    }

    private fun getAmountInSight(p: Vector<Int>): Int {
        val amount = asteroids().filter { it != p }
            .map { it.angle(p) }
            .distinct()

        return amount.size
    }

    fun print() {
        data.chunked(width)
            .map { it.map { it.toString().padStart(2, ' ') }.reduce { acc, s -> "$acc  $s" } }
            .forEach { println(it) }
    }

    fun printResult() {
        (0 until height).flatMap { y -> (0 until width).map { x -> Vector(x, y) } }
            .map { p -> p to (_amountInSight[p] ?: '.') }
            .chunked(width)
            .map { it.map { it.second.toString().padStart(2, ' ') }.reduce { acc, s -> "$acc  $s" } }
            .forEach { println(it) }
    }
}

class Vector<T : Number>(val x: T, val y: T) {
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
