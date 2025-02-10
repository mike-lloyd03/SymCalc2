<script lang="ts">
	import type { ExpressionInput, HistoryItem } from '$lib/types';
	import ActionButton from './ActionButton.svelte';

	interface Props {
		historyItem?: HistoryItem;
		input: ExpressionInput;
		deleteHistoryItem: () => void;
	}
	let { historyItem = $bindable(), input = $bindable(), deleteHistoryItem }: Props = $props();
</script>

{#if historyItem}
	<div
		class="absolute h-full w-full grid place-items-center bg-black/25"
		onclick={() => (historyItem = undefined)}
		role="none"
	>
		<div class="max-w-3/4 min-w-64 bg-zinc-800 rounded-md text-center p-4">
			<div class="mb-6 font-bold">
				{historyItem.equation} = {historyItem.solution}
			</div>
			<div class="flex flex-col gap-2">
				<ActionButton
					title="Copy Equation"
					onclick={() => {
						input.text =
							input.text.slice(0, input.cursorPos) +
							historyItem?.equation +
							input.text.slice(input.cursorPos);
						input.cursorPos += historyItem?.equation.length || 0;
					}}
				/>
				<ActionButton
					title="Copy Answer"
					onclick={() => {
						input.text =
							input.text.slice(0, input.cursorPos) +
							historyItem?.solution.toString() +
							input.text.slice(input.cursorPos);
						input.cursorPos += historyItem?.solution.toString().length || 0;
					}}
				/>
				<ActionButton title="Delete" onclick={deleteHistoryItem} />
			</div>
		</div>
	</div>
{/if}
