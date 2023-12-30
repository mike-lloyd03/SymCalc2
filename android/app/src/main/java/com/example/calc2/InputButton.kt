package com.example.calc2

import androidx.compose.material3.Button
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.MutableState
import androidx.compose.ui.text.input.TextFieldValue

enum class InputButton {
    Num_0,
    Num_1,
    Num_2,
    Num_3,
    Num_4,
    Num_5,
    Num_6,
    Num_7,
    Num_8,
    Num_9,
    Slash,
    Sqrt,
    L_Paren,
    R_Paren,
    Star,
    Carrot,
    X,
    Minus,
    ApproxEquals,
    Y,
    Dot,
    Comma,
    Plus;

    override fun toString(): String {
        return when (this) {
            Num_0 -> "0"
            Num_1 -> "1"
            Num_2 -> "2"
            Num_3 -> "3"
            Num_4 -> "4"
            Num_5 -> "5"
            Num_6 -> "6"
            Num_7 -> "7"
            Num_8 -> "8"
            Num_9 -> "9"
            Slash -> "/"
            Sqrt -> "sqrt("
            L_Paren -> "("
            R_Paren -> ")"
            Star -> "*"
            Carrot -> "^"
            Minus -> "-"
            ApproxEquals -> "≈"
            Dot -> "."
            Comma -> ","
            Plus -> "+"
            X -> "x"
            Y -> "y"
        }
    }

    private fun buttonName(): String {
        return when (this) {
            Sqrt -> "√"
            else -> this.toString()
        }
    }

    @Composable
    fun toButton(input: MutableState<TextFieldValue>) {
        val buttonName = this.buttonName()
        Button(onClick = {
            input.value = TextFieldValue(
                text = input.value.text + this.toString()
            )
        }) {
            Text(buttonName)
        }
    }
}