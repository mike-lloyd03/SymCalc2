package com.example.calc2

import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Button
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.MutableState
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.TextRange
import androidx.compose.ui.text.input.TextFieldValue
import androidx.compose.ui.unit.dp

enum class CalcButton {
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Slash,
    Sqrt,
    LParen,
    RParen,
    Star,
    Carrot,
    X,
    Minus,
    ApproxEquals,
    Y,
    Dot,
    Comma,
    Plus,
    Second,
    LArrow,
    RArrow,
    Delete,
    Func,
    Keyboard,
    Enter;

    private fun input(): String {
        return when (this) {
            Num0 -> "0"
            Num1 -> "1"
            Num2 -> "2"
            Num3 -> "3"
            Num4 -> "4"
            Num5 -> "5"
            Num6 -> "6"
            Num7 -> "7"
            Num8 -> "8"
            Num9 -> "9"
            Slash -> "/"
            Sqrt -> "sqrt("
            LParen -> "("
            RParen -> ")"
            Star -> "*"
            Carrot -> "^"
            Minus -> "-"
            ApproxEquals -> "≈"
            Dot -> "."
            Comma -> ","
            Plus -> "+"
            X -> "x"
            Y -> "y"
            else -> ""
        }
    }

    private fun buttonName(): String {
        return when (this) {
            Second -> "2nd"
            LArrow -> "<-"
            RArrow -> "->"
            Delete -> "Del"
            Func -> "Func"
            Keyboard -> "KB"
            Enter -> "Enter"
            Sqrt -> "√"
            else -> this.input()
        }
    }

    @Composable
    fun ToButton(input: MutableState<TextFieldValue>) {
        val selection = input.value.selection
        val text = input.value.text
        val newSelectionIndex = selection.end + 1

        val newText = if (selection.collapsed) {
            val beforeCursorText = text.substring(0, selection.end)
            val afterCursorText = text.substring(selection.end)
            beforeCursorText + this.input() + afterCursorText
        } else {
            // Need to handle case where text is selected
            val beforeCursorText = text.substring(0, selection.end)
            val afterCursorText = text.substring(selection.end)
            beforeCursorText + this.input() + afterCursorText
        }

        this.ToButton {
            input.value = TextFieldValue(
                text = newText,
                selection = TextRange(newSelectionIndex)
            )
        }
    }

    @Composable
    fun ToButton(onClick: () -> Unit) {
        val buttonName = this.buttonName()
        Button(
            modifier = Modifier
                .height(56.dp)
                .padding(1.dp),
            shape = RoundedCornerShape(10),
            contentPadding = PaddingValues(0.dp),
            onClick = { onClick() }
        ) {
            Text(
                text = buttonName,
                maxLines = 1,
            )
        }
    }


}