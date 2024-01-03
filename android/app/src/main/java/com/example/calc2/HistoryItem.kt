package com.example.calc2

import androidx.compose.foundation.ExperimentalFoundationApi
import androidx.compose.foundation.combinedClickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.DropdownMenu
import androidx.compose.material3.DropdownMenuItem
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.hapticfeedback.HapticFeedbackType
import androidx.compose.ui.platform.LocalHapticFeedback
import androidx.compose.ui.unit.dp
import com.example.calc2.shared.HistoryItem

@OptIn(ExperimentalFoundationApi::class)
@Composable
fun HistoryRow(
    history: HistoryItem,
    onCopyHistory: (HistoryItem) -> Unit,
    onDeleteHistory: (HistoryItem) -> Unit
) {
    val haptics = LocalHapticFeedback.current
    var expanded by remember { mutableStateOf(false) }

    Row(
        modifier = Modifier
            .fillMaxWidth()
            .combinedClickable(
                enabled = true,
                onLongClick = {
                    haptics.performHapticFeedback(HapticFeedbackType.LongPress)
                    expanded = true
                },
                onClick = {},
            ),
        horizontalArrangement = Arrangement.SpaceBetween,
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

    DropdownMenu(
        expanded = expanded,
        onDismissRequest = { expanded = false },
    ) {
        DropdownMenuItem(
            text = { Text("Copy") },
            onClick = {
                onCopyHistory(history)
                expanded = false
            }
        )
        DropdownMenuItem(
            text = { Text("Delete") },
            onClick = {
                onDeleteHistory(history)
                expanded = false
            }
        )
    }
}