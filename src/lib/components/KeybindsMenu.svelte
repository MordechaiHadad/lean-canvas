<script>
	export let id = '';

	let isOpen = false;

	export function toggleMenu() {
		if (isOpen) {
			document.getElementById(id)?.classList.replace("flex", "hidden");
		} else {
			document.getElementById(id)?.classList.replace("hidden", "flex");
		}

		isOpen = !isOpen;
	}
</script>

<div
	class="fixed inset-0 z-50 items-center justify-center hidden"
	style="opacity: {isOpen ? 1 : 0}"
	id={id}
>
	<div class="rounded-lg p-4 text-back dark:text-white bg-white1/60 dark:bg-black1/60 border-white/50 dark:border-black/50 backdrop-blur-md shadow-md">
		<h1 class="mb-2 text-lg font-bold text-center">Keybinds Menu</h1>
		<p><kbd>TAB</kbd> - Switch to the next textbox</p>
		<p><kbd>Shift</kbd> + <kbd>TAB</kbd> - Switch to the previous textbox</p>
		<button on:click={toggleMenu}>Close</button>

	</div>
</div>
