<script lang="ts">
	import '../app.css';

	const { children } = $props();
</script>

<div class="bg-zinc-900">
	{@render children()}
</div>
