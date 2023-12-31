package com.example.calc2

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.input.TextFieldValue
import androidx.compose.ui.unit.dp
import com.example.calc2.shared.Calc
import com.example.calc2.ui.theme.Calc2Theme

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        val calc = Calc()
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

@Composable
fun Calculator(calc: Calc) {
    val input = remember { mutableStateOf(TextFieldValue("")) }
    var history by remember {
        mutableStateOf(calc.getHistory())
    }

    fun evalFunction() {
        try {
            calc.evaluate(input.value.text)
            history = calc.getHistory()
            input.value = TextFieldValue("")
        } catch (e: Exception) {
            println(e)
        }
    }

    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(2.dp)
    ) {
        Spacer(modifier = Modifier.weight(1f))
        LazyColumn(
            reverseLayout = true,
            modifier = Modifier
                .weight(1f)
                .fillMaxWidth()
        ) {
            items(history.reversed()) { history ->
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.SpaceBetween
                ) {
                    Text(
                        text = history.equation,
                        modifier = Modifier.padding(8.dp)
                    )
                    Text(
                        text = history.solution.toString(),
                        modifier = Modifier.padding(8.dp)
                    )
                }
            }
        }
        TextField(
            value = input.value,
            onValueChange = { newInput ->
                input.value = newInput
            },
            singleLine = true,
            modifier = Modifier.fillMaxWidth()
        )
        ButtonGrid(input) { evalFunction() }
    }
}