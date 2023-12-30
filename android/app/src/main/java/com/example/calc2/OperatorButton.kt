package com.example.calc2

import androidx.compose.material3.Button
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable

enum class OperatorButton {
    Second,
    L_Arrow,
    R_Arrow,
    Delete,
    Func,
    Keyboard,
    Enter;

    private fun buttonName(): String {
        return when (this) {
            Second -> "2nd"
            L_Arrow -> "<-"
            R_Arrow -> "->"
            Delete -> "Del"
            Func -> "Func"
            Keyboard -> "KB"
            Enter -> "Enter"
        }
    }

    @Composable
    fun toButton(onClick: () -> Unit) {
        val buttonName = this.buttonName()
        Button(onClick = { onClick() }) {
            Text(buttonName)
        }
    }
}