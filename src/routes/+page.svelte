<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import ButtonGrid from '../lib/ButtonGrid.svelte';
	import type { ExpressionInput, HistoryItem } from '../lib/types';
	import { getHistory, scrollToBottom } from '../lib/utils';
	import HistoryRow from '$lib/HistoryRow.svelte';
	import Modal from '$lib/modal/Modal.svelte';

	let input: ExpressionInput = $state({ text: '', cursorPos: 0 });
	let history: HistoryItem[] = $state([]);
	let historyDiv: HTMLDivElement;
	let editHistoryItem: HistoryItem | undefined = $state(undefined);

	$effect(() => {
		getHistory().then((v) => {
			history = v;
			scrollToBottom(historyDiv);
		});
	});

	async function calc() {
		let solution: number = await invoke('calc', { expression: input.text });
		history.push({ equation: input.text, solution });
		input.text = '';
		input.cursorPos = 0;
		history = await getHistory();
		scrollToBottom(historyDiv);
	}

	async function deleteHistoryItem() {
		await invoke('delete_history_item', { id: editHistoryItem?.id || 0 });
		history = await getHistory();
		scrollToBottom(historyDiv);
	}
</script>

<main class="relative container h-screen text-white">
	<div class="absolute h-screen w-full flex flex-col p-2">
		<div class="overflow-y-auto flex-1 content-end" bind:this={historyDiv}>
			{#each history as item}
				<HistoryRow {item} bind:input bind:editHistoryItem />
			{/each}
		</div>

		<div class="grid bg-zinc-800 p-2 my-1 rounded-sm">
			<input class="col-start-1 row-start-1 bg-none" bind:value={input.text} />
			<div class="col-start-1 row-start-1">
				<span class="text-transparent">{input.text.slice(0, input.cursorPos)}</span><span>|</span>
			</div>
		</div>

		<ButtonGrid bind:input onsubmit={calc} />
	</div>

	<Modal bind:historyItem={editHistoryItem} bind:input {deleteHistoryItem} />
</main>
