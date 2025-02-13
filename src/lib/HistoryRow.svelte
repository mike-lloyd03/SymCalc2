<script lang="ts">
	import type { ExpressionInput, HistoryItem } from './types';

	interface Props {
		item: HistoryItem;
		input: ExpressionInput;
		editHistoryItem?: HistoryItem;
	}

	let { item, input = $bindable(), editHistoryItem = $bindable() }: Props = $props();
</script>

<div
	class="flex justify-between my-2 active:bg-zinc-800 rounded-sm px-1"
	onclick={() => {
		editHistoryItem = item;
	}}
	role="none"
>
	<div>{item.equation}</div>
	<div>{item.solution || ' '}</div>
</div>
