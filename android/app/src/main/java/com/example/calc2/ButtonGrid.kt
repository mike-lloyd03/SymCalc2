package com.example.calc2

import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.lazy.grid.GridCells
import androidx.compose.foundation.lazy.grid.LazyVerticalGrid
import androidx.compose.foundation.lazy.grid.items
import androidx.compose.runtime.Composable
import androidx.compose.runtime.MutableState
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.TextRange
import androidx.compose.ui.text.input.TextFieldValue

@Composable
fun ButtonGrid(input: MutableState<TextFieldValue>, evalFunction: () -> Unit) {
    fun moveCursor(amount: Int) {
        val currentPos = input.value.selection.end
        var newPos = currentPos + amount
        if (newPos < 0) {
            newPos = 0
        }

        input.value = TextFieldValue(
            text = input.value.text,
            selection = TextRange(newPos)
        )
    }

    fun handleDelete() {
        val selection = input.value.selection
        val text = input.value.text
        var newSelectionIndex = selection.end.minus(1).coerceAtLeast(0)

        val newText = if (selection.collapsed) {
            val beforeCursorText = text.substring(
                0,
                selection.end
                    .minus(1)
                    .coerceAtLeast(0)
            )
            val afterCursorText = text.substring(selection.end)
            beforeCursorText + afterCursorText
        } else {
            val beforeCursorText = text.substring(0, selection.start)
            val afterCursorText = text.substring(selection.end)
            newSelectionIndex = selection.start
            beforeCursorText + afterCursorText
        }

        input.value = TextFieldValue(
            text = newText,
            selection = TextRange(newSelectionIndex)
        )
    }

    val buttons = listOf(
        CalcButton.Second,
        CalcButton.LParen,
        CalcButton.RParen,
        CalcButton.LArrow,
        CalcButton.RArrow,
        CalcButton.Delete,
        CalcButton.Func,
        CalcButton.Num7,
        CalcButton.Num8,
        CalcButton.Num9,
        CalcButton.Slash,
        CalcButton.Sqrt,
        CalcButton.Keyboard,
        CalcButton.Num4,
        CalcButton.Num5,
        CalcButton.Num6,
        CalcButton.Star,
        CalcButton.Carrot,
        CalcButton.X,
        CalcButton.Num1,
        CalcButton.Num2,
        CalcButton.Num3,
        CalcButton.Minus,
        CalcButton.ApproxEquals,
        CalcButton.Y,
        CalcButton.Num0,
        CalcButton.Dot,
        CalcButton.Comma,
        CalcButton.Plus,
        CalcButton.Enter,
    )

    LazyVerticalGrid(
        modifier = Modifier.fillMaxWidth(),
        columns = GridCells.Fixed(6),
        userScrollEnabled = false,
    ) {
        items(buttons) { button ->
            when (button) {
                CalcButton.Enter -> button.ToButton(evalFunction)
                CalcButton.LArrow -> button.ToButton { moveCursor(-1) }
                CalcButton.RArrow -> button.ToButton { moveCursor(1) }
                CalcButton.Delete -> button.ToButton { handleDelete() }

                else -> button.ToButton(input)
            }
        }
    }
}