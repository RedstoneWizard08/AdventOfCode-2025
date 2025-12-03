<script lang="ts">
    import type { ProblemPart } from "./problems";

    interface Props {
        part: ProblemPart;
        number: number;
    }

    const { part, number }: Props = $props();
    const nanos = $derived(part.new.mean.point_estimate);
    const micros = $derived(nanos / 1000);
    const millis = $derived(micros / 1000);
    const secs = $derived(millis / 1000);

    const time = $derived(
        nanos < 1000
            ? `${nanos.toFixed(2)} ns`
            : micros < 1000
              ? `${micros.toFixed(2)} µs`
              : millis < 1000
                ? `${millis.toFixed(2)} ms`
                : `${secs.toFixed(2)} s`,
    );
</script>

<div class="p-4 mt-4 bg-dark-1">
    <p class="m-0 p-0">Part {number}</p>
    <p class="m-0 p-0 text-gray-3">Runtime: <span class="text-green">{time}</span></p>
</div>
