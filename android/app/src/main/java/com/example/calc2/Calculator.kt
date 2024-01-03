package com.example.calc2

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.TextField
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.TextRange
import androidx.compose.ui.text.input.TextFieldValue
import androidx.compose.ui.unit.dp
import com.example.calc2.shared.Calc
import com.example.calc2.shared.HistoryItem

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

    fun deleteHistoryItem(historyItem: HistoryItem) {
        logger.info { "Deleting history item $historyItem.id" }
        if (historyItem.id != null) {
            calc.deleteHistory(historyItem.id!!)
            history = calc.getHistory()
        }
    }

    fun copyHistoryItem(historyItem: HistoryItem) {
        input.value = TextFieldValue(
            text = historyItem.equation,
            selection = TextRange(historyItem.equation.length)
        )
    }

    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(2.dp)
    ) {
        Spacer(modifier = Modifier)
        LazyColumn(
            reverseLayout = true,
            modifier = Modifier
                .weight(1f)
                .fillMaxWidth()
        ) {
            items(history.reversed()) { history ->
                HistoryRow(history, ::copyHistoryItem, ::deleteHistoryItem)
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
        ButtonGrid(input, ::evalFunction)
    }
}
