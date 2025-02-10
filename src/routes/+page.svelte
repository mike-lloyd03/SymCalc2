<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import ButtonGrid from '../lib/ButtonGrid.svelte';
	import type { ExpressionInput, HistoryItem } from '../lib/types';
	import { getHistory } from '../lib/utils';

	let input: ExpressionInput = $state({ text: '', cursorPos: 0 });
	let history: HistoryItem[] = $state([]);
	let historyDiv: HTMLDivElement;

	$effect(() => {
		getHistory().then((v) => {
			history = v;
		});
	});

	async function calc() {
		let solution: number = await invoke('calc', { expression: input.text });
		history.push({ equation: input.text, solution });
		input.text = '';
		input.cursorPos = 0;
		history = await getHistory();
		historyDiv.scroll({ top: historyDiv.scrollHeight, behavior: 'smooth' });
	}
</script>

<main class="container h-screen flex flex-col text-white p-2">
	<div class="overflow-y-auto flex-1 content-end" bind:this={historyDiv}>
		{#each history as item}
			<div class="flex justify-between my-1">
				<div>{item.equation}</div>
				<div>{item.solution}</div>
			</div>
		{/each}
	</div>

	<div class="grid">
		<input class="col-start-1 row-start-1 bg-none" bind:value={input.text} />
		<div class="col-start-1 row-start-1">
			<span class="text-transparent">{input.text.slice(0, input.cursorPos)}</span><span>|</span>
		</div>
	</div>

	<ButtonGrid bind:input onsubmit={calc} />
</main>
