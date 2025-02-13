<script lang="ts">
	import Button from './Button.svelte';
	import type { ExpressionInput } from './types';

	interface Props {
		input: ExpressionInput;
		onsubmit: () => void;
	}

	let { input = $bindable(), onsubmit }: Props = $props();

	function del() {
		input.text = input.text.slice(0, input.cursorPos - 1) + input.text.slice(input.cursorPos);
		let newPos = input.cursorPos - 1;
		if (newPos < 0) {
			newPos = 0;
		}
		input.cursorPos = newPos;
	}

	function move(amt: number) {
		let newPos = (input.cursorPos += amt);
		if (newPos < 0) {
			newPos = 0;
		}
		if (newPos > input.text.length) {
			newPos = input.text.length;
		}
		input.cursorPos = newPos;
	}
</script>

<div class="grid grid-cols-6 grid-rows-5 gap-2">
	<Button bind:input label="2nd" />
	<Button bind:input label="(" />
	<Button bind:input label=")" />
	<Button bind:input label="<-" onclick={() => move(-1)} />
	<Button bind:input label="->" onclick={() => move(1)} />
	<Button bind:input label="Del" onclick={del} />

	<Button bind:input label="func" value="" />
	<Button bind:input label="7" />
	<Button bind:input label="8" />
	<Button bind:input label="9" />
	<Button bind:input label="/" />
	<Button bind:input label="sqrt" value="sqrt(" />

	<Button bind:input label="key" value="" />
	<Button bind:input label="4" />
	<Button bind:input label="5" />
	<Button bind:input label="6" />
	<Button bind:input label="*" />
	<Button bind:input label="^" />

	<Button bind:input label="x" />
	<Button bind:input label="1" />
	<Button bind:input label="2" />
	<Button bind:input label="3" />
	<Button bind:input label="-" />
	<Button bind:input label="=" />

	<Button bind:input label="y" />
	<Button bind:input label="0" />
	<Button bind:input label="." />
	<Button bind:input label="," />
	<Button bind:input label="+" />
	<Button label="Enter" onclick={onsubmit} />
</div>
