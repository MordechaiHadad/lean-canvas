<script lang="ts">
	import Section from '$lib/components/item.svelte';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { open } from '@tauri-apps/api/dialog';
	let isDarkTheme: boolean;

	onMount(() => {
		isDarkTheme = window.matchMedia('(prefers-color-scheme: dark)').matches;
	});

	function toggle_theme() {
		isDarkTheme = !isDarkTheme;
		document.documentElement.classList.toggle('dark', isDarkTheme);
	}

	function save() {
		const ids = ['problem', 'solution'];
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

		invoke('test_command', { values: values, ids: existing_ids });
	}

	async function load() {
		const selected = await open({ multiple: false });
		console.log(selected);
	}
</script>

<div>
	<div class="toolbar mt-3 items-center justify-center gap-12">
		<div
			class="fa-solid fa-house text-center text-xl text-black transition duration-300 ease-in-out hover:text-primary dark:text-white"
		/>
		<div
			contenteditable
			class="rounded-md bg-white1 p-1 px-2 text-black outline-none dark:bg-black1 dark:text-white"
		>
			Unnamed
		</div>
		<div class="group cursor-pointer">
			<i class="fa-solid fa-bars text-center text-xl text-black dark:text-white" />
			<div
				class="absolute hidden grid-cols-2 gap-2 rounded-md border border-white/10 bg-white1/50 p-2 text-black backdrop-blur-md group-hover:grid dark:border-black/10 dark:bg-black1/10 dark:text-white"
				id="dropdown"
			>
				<div class="row-span-2 flex flex-col overflow-hidden rounded-md ">
					<!-- svelte-ignore a11y-click-events-have-key-events -->
					<p class="bg-white p-2 hover:bg-white/50 dark:bg-black dark:hover:bg-black/50">New</p>
					<p
						class="bg-white p-2 hover:bg-white/50 dark:bg-black dark:hover:bg-black/50"
						on:click={load}
					>
						Open
					</p>
					<p
						class="border border-primary bg-white p-2 hover:bg-white/50 dark:bg-black dark:hover:bg-black/50"
						on:click={save}
					>
						Save
					</p>
					<p class="bg-white p-2 hover:bg-white/50 dark:bg-black dark:hover:bg-black/50">Save As</p>
					<p
						class="border border-primary bg-white p-2 hover:bg-white/50 dark:bg-black dark:hover:bg-black/50"
					>
						About
					</p>
				</div>
				<!-- svelte-ignore a11y-click-events-have-key-events -->
				<div
					class="grid gap-1 rounded-md bg-white px-2 py-2 transition duration-200 ease-in-out hover:bg-white/50 dark:bg-black dark:hover:bg-black/50"
					on:click={toggle_theme}
				>
					<i class="fa-solid fa-sun text-center" />
					<p class="text-center">Toggle Theme</p>
				</div>
				<div
					class="grid gap-1 rounded-md bg-white px-2 py-2 transition duration-200 ease-in-out hover:bg-white/50 dark:bg-black dark:hover:bg-black/50"
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
