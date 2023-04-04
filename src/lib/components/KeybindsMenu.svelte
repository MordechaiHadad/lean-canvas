<script>
	export let id = '';

	let isOpen = false;

	function toggleMneu() {
		if (isOpen) {
			document.getElementById(id)?.classList.remove('overflow-hidden');
		} else {
			document.getElementById(id)?.classList.add('overflow-hidden');
		}

		isOpen = !isOpen;
	}
</script>

<div
	class="bg-gray-800 fixed inset-0 z-50 flex items-center justify-center overflow-hidden bg-opacity-50 transition-opacity duration-300"
	style="opacity: {isOpen ? 1 : 0}"
	{id}
>
	<div class="rounded-lg bg-white p-4">
		<h1 class="mb-2 text-lg font-bold">Menu</h1>
		<ul>
			<li class="mb-2">
				<a href="#" class="text-gray-800 hover:text-gray-700">Item 1</a>
			</li>
			<li class="mb-2">
				<a href="#" class="text-gray-800 hover:text-gray-700">Item 2</a>
			</li>
			<li class="mb-2">
				<a href="#" class="text-gray-800 hover:text-gray-700">Item 3</a>
			</li>
		</ul>
	</div>
</div>
