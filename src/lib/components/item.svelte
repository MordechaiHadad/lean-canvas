<script lang="ts">
	export let classes: string = '';
	export let section_name: string = '';
	export let id: string;
</script>

<div class="{classes} flex flex-col gap-y-4 rounded-md justify-items-center bg-white1 dark:bg-black1 px-4 py-2" id={id}>
	<h1 class="text-sm font-normal text-black dark:text-white select-none">{section_name}</h1>
    <textarea class="resize-none border-2 border-white bg-white dark:bg-black dark:border-black rounded-md text-base font-medium w-full h-full py-2 px-3 text-black dark:text-white leading-tight focus:border-primary dark:focus:border-primary focus:border-2 focus:outline-none" rows="5"></textarea>
</div>
