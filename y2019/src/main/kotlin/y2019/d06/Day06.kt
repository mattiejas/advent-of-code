class Planet(val name: String, val orbits: String)

class Day06 : Day {
    private val planets = Utils.readFile("2019/06/input")
        .map { p -> p.split(')') }
        .map { p -> Planet(p[1], p[0]) }
        .groupBy { it.name }

    override fun partOne() {
        val amount = planets.map { orbitsAmount(it.key) }.sum()
        println(amount)
    }

    override fun partTwo() {
        val route = findRoute("YOU", "SAN")
        println(route.size - 3)
    }

    private fun orbitsAmount(planet: String, count: Int = 0): Int {
        if (planet == "COM") return count
        return planets[planet]?.map { p -> orbitsAmount(p.orbits, count + 1) }?.sum() ?: count
    }

    private fun routeToCOM(planet: String, route: List<String> = emptyList()): List<String>? {
        if (planet == "COM") return route
        if (planets[planet] == null) return null
        return planets[planet]!!.mapNotNull { p -> routeToCOM(p.orbits, route.plus(planet)) }.first()
    }

    private fun findRoute(from: String, to: String): List<String> {
        val routeA = routeToCOM(from) ?: emptyList()
        val routeB = routeToCOM(to) ?: emptyList()

        val complement = routeA.union(routeB).minus(routeA.intersect(routeB))
        val intersection = routeA.intersect(routeB).last()
        return complement.toList().plus(intersection)
    }
}
