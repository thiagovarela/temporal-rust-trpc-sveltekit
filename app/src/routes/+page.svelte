<script lang="ts">
	import { page } from '$app/stores';
	import { trpc } from '$lib/trpc/client';

	let greeting: string;
	let loading = false;

	const loadData = async () => {
		loading = true;
		let h = await crypto.randomUUID();
		const { userId } = await trpc($page).auth.signUp.mutate({
			first_name: 'Testing',
			last_name: '123',
			email: `thiagovarela@gmail.com`,
			password: 'anothertest'
		});
		console.log(userId);
		loading = false;
	};
</script>

<h6>Loading data in<br /><code>+page.svelte</code></h6>

<a
	href="#load"
	role="button"
	class="secondary"
	aria-busy={loading}
	on:click|preventDefault={loadData}>Load</a
>
<p>{greeting}</p>
