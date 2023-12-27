package com.example.calc2

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableDoubleStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import com.example.calc2.shared.evaluate
import com.example.calc2.ui.theme.Calc2Theme

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            Calc2Theme {
                // A surface container using the 'background' color from the theme
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {
                    InputField()
                }
            }
        }
    }
}

@Composable
fun InputField() {
    var input by remember { mutableStateOf("") }
    var answer by remember {
        mutableDoubleStateOf(0.0)
    }

    Column {
        Row {
            TextField(
                value = input,
                onValueChange = { input = it },
                singleLine = true,
            )

            Button(
                onClick = {
                    try {
                        answer = evaluate(input)
                    } catch (e: Exception) {
                        println(e)
                    }

                },
            ) {
                Text("Evaluate")
            }
        }
        Text(answer.toString())
    }

}

@Preview
@Composable
fun InputFieldPreview() {
    Calc2Theme {
        InputField()
    }
}