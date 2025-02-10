<script lang="ts">
	interface Props {
		title: string;
		onclick: () => void;
	}
	let { title, onclick }: Props = $props();
</script>

<button class="rounded-lg active:bg-zinc-700" {onclick}>{title}</button>
