<script lang="ts">
	import Section from '$lib/components/item.svelte';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	let isDarkTheme: boolean;
	const ids = [
		'problem',
		'solution',
		'metrics',
		'value',
		'advantage',
		'channels',
		'customers',
		'budget',
		'revenue'
	];
	onMount(() => {
		isDarkTheme = window.matchMedia('(prefers-color-scheme: dark)').matches;
	});

	function toggle_theme() {
		isDarkTheme = !isDarkTheme;
		document.documentElement.classList.toggle('dark', isDarkTheme);
	}

	function save() {
		let values: string[] = [];
		let existing_ids: string[] = [];

		ids.forEach((id) => {
			const parent = document.getElementById(id);

			if (!parent) {
				return;
			}

			const textarea = parent.querySelector('textarea');

			if (!textarea) {
				return;
			}

			existing_ids.push(id);
			values.push(textarea.value);
		});

		const prjName = document.getElementById('main_name');

		if (!prjName) {
			return;
		}

		invoke('save_file', {
			values: values,
			ids: existing_ids,
			name: prjName.textContent
		});
	}

	async function load() {
		const content = await invoke('read_file');
		if (content.data === '') {
			return;
		}
		const json = JSON.parse(content.data);
		const map = new Map(Object.entries(json));

		map.forEach((v, k) => {
			const parent = document.getElementById(k);

			if (!parent) {
				return;
			}

			let textarea = parent.querySelector('textarea');

			if (!textarea) {
				return;
			}

			textarea.value = v as string;
		});

		let name = document.getElementById('main_name');
		name.textContent = content.name;
	}
</script>

<div>
	<div class="toolbar mt-3 items-center justify-center gap-12">
		<div
			class="fa-solid fa-house text-center text-xl text-black transition duration-300 ease-in-out hover:text-primary dark:hover:text-primary dark:text-white"
		/>
		<div
			contenteditable
			class="rounded-md bg-white1 p-1 px-2 text-black outline-none dark:bg-black1 dark:text-white"
			id="main_name"
		>
			Unnamed
		</div>
		<div class="group cursor-pointer">
			<i class="fa-solid fa-bars text-center text-xl text-black dark:text-white" />
			<div
				class="absolute hidden grid-cols-2 gap-2 rounded-md border border-white/10 bg-white1/50 p-2 text-black backdrop-blur-md group-hover:grid dark:border-black/10 dark:bg-black1/10 dark:text-white"
				id="dropdown"
			>
				<div class="row-span-2 flex flex-col overflow-hidden rounded-md shadow-md backdrop-blur-md">
					<!-- svelte-ignore a11y-click-events-have-key-events -->
					<p class="p-2 hover:bg-white/70 dark:hover:bg-black/70">New</p>
					<p class="p-2 hover:bg-white/70 dark:hover:bg-black/70" on:click={load}>Open</p>
					<p class="p-2 hover:bg-white/70 dark:hover:bg-black/70" on:click={save}>Save</p>
					<p class="p-2 hover:bg-white/70 dark:hover:bg-black/70">Save As</p>
					<p class="p-2 hover:bg-white/70 dark:hover:bg-black/70">About</p>
				</div>
				<!-- svelte-ignore a11y-click-events-have-key-events -->
				<div
					class="grid gap-1 rounded-md px-2 py-2 shadow-md backdrop-blur-md transition duration-200 ease-in-out hover:bg-white/70 dark:hover:bg-black/70"
					on:click={toggle_theme}
				>
					<i class="fa-solid fa-sun text-center" />
					<p class="text-center">Toggle Theme</p>
				</div>
				<div
					class="grid gap-1 rounded-md px-2 py-2 shadow-md backdrop-blur-md transition duration-200 ease-in-out hover:bg-white/70 dark:hover:bg-black/70"
				>
					<i class="fa-solid fa-keyboard text-center" />
					<p class="text-center">Keybinds</p>
				</div>
			</div>
		</div>
	</div>

	<div class="grid h-screen gap-1 p-4">
		<div class="row-span-2 grid h-full auto-cols-fr grid-flow-col grid-rows-2 gap-2 gap-y-1">
			<Section classes={'row-span-2 col-span-1'} id={'problem'} section_name={'Problem'} />
			<Section section_name={'Solution'} id={'solution'} />
			<Section section_name={'Key Metrics'} id={'metrics'} />
			<Section classes={'row-span-2'} id={'value'} section_name={'Value Proposition'} />
			<Section section_name={'Unfair Advantage'} id={'advantage'} />
			<Section section_name={'Channels'} id="channels" />
			<Section classes={'row-span-2'} section_name={'Customer Segments'} id={'customers'} />
		</div>
		<div class="row-span-1 grid auto-cols-fr grid-flow-col gap-2">
			<Section section_name={'Cost Structure'} id={'budget'} />
			<Section section_name={'Revenue Streams'} id={'revenue'} />
		</div>
	</div>
</div>

<style>
	.toolbar {
		display: grid;
		grid-template-columns: min-content max-content min-content;
	}
</style>
