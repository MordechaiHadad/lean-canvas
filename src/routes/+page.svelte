<script lang="ts">
	import Section from '$lib/components/item.svelte';
	import { ThemeHandler, LanguageHandler, DocumentFile } from '$lib/utils.js';
	import { onMount } from 'svelte';
	import en from "../locales/en.json";

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

	let themeHandler: ThemeHandler; 
	let languageHandler: LanguageHandler;

	let language_strings = en;

	onMount(() => {
		const isDarkTheme = window.matchMedia('(prefers-color-scheme: dark)').matches;
		themeHandler = new ThemeHandler(isDarkTheme);

		languageHandler = new LanguageHandler();
	});

	function toggleLanguage() {
		languageHandler.toggle();
		language_strings = languageHandler.language_strings;
	}

	let file: DocumentFile;
	async function loadFile() {
		file = await DocumentFile.readFile();
	}
</script>

<div>
	<div class="toolbar mt-3 items-center justify-center gap-12">
		<div
			class="fa-solid fa-house text-center text-xl text-black transition duration-300 ease-in-out hover:text-primary dark:text-white dark:hover:text-primary"
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
				class="absolute hidden grid-cols-2 grid-rows-3 gap-2 rounded-md border border-white/10 bg-white1/50 p-2 text-black backdrop-blur-md group-hover:grid dark:border-black/10 dark:bg-black1/10 dark:text-white"
				id="dropdown"
			>
				<div class="row-span-3 flex flex-col overflow-hidden rounded-md shadow-md backdrop-blur-md">
					<!-- svelte-ignore a11y-click-events-have-key-events -->
					<p class="h-11 p-2 hover:bg-white/70 dark:hover:bg-black/70">New</p>
					<p class="h-11 p-2 hover:bg-white/70 dark:hover:bg-black/70" on:click={loadFile}>Open</p>
					<p class="h-11 p-2 hover:bg-white/70 dark:hover:bg-black/70" on:click={() => file.saveFile(ids)}>Save</p>
					<p class="h-11 p-2 hover:bg-white/70 dark:hover:bg-black/70">Save As</p>
					<p class="h-11 p-2 hover:bg-white/70 dark:hover:bg-black/70">About</p>
				</div>
				<!-- svelte-ignore a11y-click-events-have-key-events -->
				<div
					class="grid gap-1 rounded-md px-2 py-2 shadow-md backdrop-blur-md transition duration-200 ease-in-out hover:bg-white/70 dark:hover:bg-black/70"
					on:click={themeHandler.toggle}
				>
					<i class="fa-solid fa-sun text-center" />
					<p class="text-center">Toggle Theme</p>
				</div>
				<div
					class="grid gap-1 rounded-md px-2 py-2 shadow-md backdrop-blur-md transition duration-200 ease-in-out hover:bg-white/70 dark:hover:bg-black/70"
					on:click={toggleLanguage}
				>
					<p class="text-center">🇺🇸</p>
					<p class="text-center">Change Language</p>
				</div>
				<div
					class="col-start-2 grid gap-1 rounded-md px-2 py-2 shadow-md backdrop-blur-md transition duration-200 ease-in-out hover:bg-white/70 dark:hover:bg-black/70"
				>
					<i class="fa-solid fa-keyboard text-center" />
					<p class="text-center">Keybinds</p>
				</div>
			</div>
		</div>
	</div>

	<div class="grid h-screen gap-1 p-4">
		<div class="row-span-2 grid h-full auto-cols-fr grid-flow-col grid-rows-2 gap-2 gap-y-1">
			<Section
				classes={'row-span-2 col-span-1'}
				id={'problem'}
				section_name={language_strings.problem_header}
			/>
			<Section section_name={language_strings.solution_header} id={'solution'} />
			<Section section_name={language_strings.metrics_header} id={'metrics'} />
			<Section classes={'row-span-2'} id={'value'} section_name={language_strings.value_header} />
			<Section section_name={language_strings.advantage_header} id={'advantage'} />
			<Section section_name={language_strings.channels_header} id="channels" />
			<Section
				classes={'row-span-2'}
				section_name={language_strings.customers_header}
				id={'customers'}
			/>
		</div>
		<div class="row-span-1 grid auto-cols-fr grid-flow-col gap-2">
			<Section section_name={language_strings.budget_header} id={'budget'} />
			<Section section_name={language_strings.revenue_header} id={'revenue'} />
		</div>
	</div>
</div>

<style>
	.toolbar {
		display: grid;
		grid-template-columns: min-content max-content min-content;
	}
</style>
