<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import ButtonGrid from '../lib/ButtonGrid.svelte';
	import type { ExpressionInput, HistoryItem } from '../lib/types';

	let input: ExpressionInput = $state({ text: '', cursorPos: 0 });
	let result = $state('');

	let history: HistoryItem[] = $state([]);

	async function calc() {
		result = await invoke('calc', { expression: input.text });
		history.push({ expression: input.text, result });
		input.text = '';
		input.cursorPos = 0;
	}
</script>

<main class="container h-screen flex flex-col text-white p-2">
	<div class="overflow-y-auto flex-1 content-end">
		{#each history as item}
			<div class="flex justify-between my-1">
				<div>{item.expression}</div>
				<div>{item.result}</div>
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
