<script lang="ts">
    import Problem from "./Problem.svelte";
    import { problems } from "./problems";
</script>

<div class="p-4">
    <p class="m-0 p-6 text-xl font-bold px-12">
        Redstone's Advent of Code 2025 Solution Benchmarks
    </p>

    {#each problems as problem}
        <Problem {problem} />
    {/each}
</div>
