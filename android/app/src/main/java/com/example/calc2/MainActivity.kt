package com.example.calc2

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.ui.Modifier
import com.example.calc2.shared.Calc
import com.example.calc2.ui.theme.Calc2Theme
import io.github.oshai.kotlinlogging.KotlinLogging

val logger = KotlinLogging.logger {}

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        val calc = Calc(filesDir.toString())
        logger.info { "Starting app" }

        super.onCreate(savedInstanceState)
        setContent {
            Calc2Theme {
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {
                    Calculator(calc)
                }
            }
        }
    }
}