package y2019.d13

import Vector
import com.badlogic.gdx.ApplicationAdapter
import com.badlogic.gdx.Gdx
import com.badlogic.gdx.Input
import com.badlogic.gdx.InputAdapter
import com.badlogic.gdx.backends.lwjgl.LwjglApplication
import com.badlogic.gdx.backends.lwjgl.LwjglApplicationConfiguration
import com.badlogic.gdx.graphics.Color
import com.badlogic.gdx.graphics.GL20
import com.badlogic.gdx.graphics.OrthographicCamera
import com.badlogic.gdx.graphics.glutils.ShapeRenderer
import java.time.temporal.TemporalAdjusters.previous
import java.util.*


object ArcadeRunner {
    class Game(val arcade: Arcade) : ApplicationAdapter() {
        private lateinit var camera: OrthographicCamera
        private lateinit var shape: ShapeRenderer

        val tileSize = 16f
        lateinit var dimensions: Vector<Int>

        var previousTime = Date().time
        var lag = 0f
        val MS_PER_UPDATE = 16

//        val listener: InputAdapter = object : InputAdapter() {
//            override fun keyDown(keycode: Int): Boolean {
//                when(keycode) {
//                    Input.Keys.LEFT -> arcade.input(-1)
//                    Input.Keys.RIGHT -> arcade.input(1)
//                    Input.Keys.SPACE -> {}
//                    else -> return false
//                }
//                arcade.run()
//                return true
//            }
//        }

        private fun handleInput() {
            val ball = arcade.ball()
            val paddle = arcade.paddle()

            if (paddle.x - ball.x > 0) {
                arcade.input(-1) // move left
            } else if (paddle.x - ball.x < 0) {
                arcade.input(1) // move right
            } else {
                arcade.input(0) // don't move
            }

            arcade.run() // continue
        }

        override fun create() {
            super.create()
            camera = OrthographicCamera()
            camera.setToOrtho(true, 800f, 480f)
            shape = ShapeRenderer()

            arcade.run() // runs till first time input
//            Gdx.input.inputProcessor = listener
        }

        private fun update() {
            dimensions = arcade.dimensions()
            camera.setToOrtho(true, dimensions.x * tileSize, dimensions.y * tileSize)

            if (arcade.paused()) {
                handleInput()
            }
        }

        override fun render() {
            Gdx.gl.glClearColor(1f, 1f, 1f, 1f)
            Gdx.gl.glClear(GL20.GL_COLOR_BUFFER_BIT)

            val current = Date().time
            val elapsed = current - previousTime
            previousTime = current
            lag += elapsed.toFloat()

            while (lag >= MS_PER_UPDATE) {
                update()
                lag -= MS_PER_UPDATE
            }

            arcade.canvas.forEach {
                shape.projectionMatrix = camera.combined
                shape.begin(ShapeRenderer.ShapeType.Filled)
                shape.color = when (it.value) {
                    0 -> Color.WHITE
                    1 -> Color.BLACK
                    2 -> Color.PINK
                    3 -> Color.CYAN
                    4 -> Color.DARK_GRAY
                    else -> Color.WHITE
                }
                shape.rect(it.key.x.toFloat() * tileSize, it.key.y.toFloat() * tileSize, tileSize, tileSize)
                shape.end()
            }
        }
    }

    fun run(arcade: Arcade) {
        val config = LwjglApplicationConfiguration()
        config.title = "Arcade"
        LwjglApplication(Game(arcade), config)
    }
}
