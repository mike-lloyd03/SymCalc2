<script lang="ts">
	import type { ExpressionInput } from './types';

	interface Props {
		label: string;
		value?: string;
		input?: ExpressionInput;
		onclick?: () => void;
	}

	let { label, value = label, input = $bindable(), onclick }: Props = $props();

	function action() {
		if (onclick) {
			console.log('Running onclick');
			onclick();
		} else if (input && value) {
			input.text = input.text.slice(0, input.cursorPos) + value + input.text.slice(input.cursorPos);
			input.cursorPos += value.length;
			console.log('Running input');
		} else {
			console.log('Default');
		}
	}
</script>

<button
	class="aspect-square bg-zinc-800 text-white rounded-md flex justify-center items-center hover:bg-zinc-700 active:bg-zinc-600 transition-colors"
	onclick={action}
>
	{label}
</button>
