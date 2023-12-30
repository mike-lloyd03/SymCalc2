package com.example.calc2

import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.lazy.grid.GridCells
import androidx.compose.foundation.lazy.grid.LazyVerticalGrid
import androidx.compose.runtime.Composable
import androidx.compose.runtime.MutableState
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.input.TextFieldValue

@Composable
fun ButtonGrid(input: MutableState<TextFieldValue>, evalFunction: () -> Unit) {
    LazyVerticalGrid(
        modifier = Modifier.fillMaxWidth(),
        columns = GridCells.Fixed(6),
        userScrollEnabled = false,
    ) {
        item { OperatorButton.Second.toButton {} }
        item { InputButton.L_Paren.toButton(input = input) }
        item { InputButton.R_Paren.toButton(input = input) }
        item { OperatorButton.L_Arrow.toButton {} }
        item { OperatorButton.R_Arrow.toButton {} }
        item { OperatorButton.Delete.toButton {} }
        item { OperatorButton.Func.toButton {} }
        item { InputButton.Num_7.toButton(input = input) }
        item { InputButton.Num_8.toButton(input = input) }
        item { InputButton.Num_9.toButton(input = input) }
        item { InputButton.Slash.toButton(input = input) }
        item { InputButton.Sqrt.toButton(input = input) }
        item { OperatorButton.Keyboard.toButton {} }
        item { InputButton.Num_4.toButton(input = input) }
        item { InputButton.Num_5.toButton(input = input) }
        item { InputButton.Num_6.toButton(input = input) }
        item { InputButton.Star.toButton(input = input) }
        item { InputButton.Carrot.toButton(input = input) }
        item { InputButton.X.toButton(input = input) }
        item { InputButton.Num_1.toButton(input = input) }
        item { InputButton.Num_2.toButton(input = input) }
        item { InputButton.Num_3.toButton(input = input) }
        item { InputButton.Minus.toButton(input = input) }
        item { InputButton.ApproxEquals.toButton(input = input) }
        item { InputButton.Y.toButton(input = input) }
        item { InputButton.Num_0.toButton(input = input) }
        item { InputButton.Dot.toButton(input = input) }
        item { InputButton.Comma.toButton(input = input) }
        item { InputButton.Plus.toButton(input = input) }
        item { OperatorButton.Enter.toButton(evalFunction) }
    }
}