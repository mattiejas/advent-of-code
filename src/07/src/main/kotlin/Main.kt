import kotlinx.coroutines.GlobalScope
import kotlinx.coroutines.launch
import kotlinx.coroutines.runBlocking
import java.nio.file.Paths

class Config {
    val inputURL = javaClass.getResource("input")
    val input = Paths.get(inputURL.toURI()).toFile()
        .readLines()
        .map { l -> l.split(',') }
        .flatten()
        .map { i -> i.toInt() }
}

fun main() {
    println(findMaxOutput(5..9))
}

fun findMaxOutput(settingsRange: IntRange = 0..4): Int {
    val outputs = mutableListOf<Int>()
    for (i in settingsRange) {
        for (j in settingsRange) {
            for (k in settingsRange) {
                for (l in settingsRange) {
                    for (m in settingsRange) {
                        if (listOf(i, j, k, l, m).distinct().size != 5) continue
                        val array = AmpArray(Config().input, listOf(i, j, k, l, m))
                        outputs.add(array.run())
                    }
                }
            }
        }
    }
    return outputs.max()!!
}
