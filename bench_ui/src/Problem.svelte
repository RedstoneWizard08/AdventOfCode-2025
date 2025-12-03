<script lang="ts">
    import Part from "./Part.svelte";
    import type { Problem } from "./problems";

    interface Props {
        problem: Problem;
    }

    const { problem }: Props = $props();
</script>

<div class="p-6 bg-dark m-4">
    <p class="text-lg p-0 m-0">Problem {problem.day}: <span class="text-cyan">{problem.name}</span></p>
    <p class="text-sm p-0 m-0 text-coolgray">December {problem.day}, 2025</p>

    <Part part={problem.part1} number={1} />
    <Part part={problem.part2} number={2} />
</div>
