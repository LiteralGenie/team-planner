<script lang="ts">
    export let activeClose = false
    export let activeMid = false
    export let activeLong = false
</script>

<svg
    viewBox="-94.5 -6 100 100"
    xmlns="http://www.w3.org/2000/svg"
    {...$$props}
>
    <path
        class="arrow"
        d="M -70.7107 70.7107 L -21.2132 7.0711 l -9.1924 -9.1924 L 3.5355 -3.5355 L 2.1213 30.4056 l -9.1924 -9.1924 Z"
    />

    <g class:active={activeLong}>
        <path
            d="M -17.6777 123.7437 A 75 75 45 0 0 -4.6669 35.0725 L -12.9754 40.7647 A 65 65 45 0 1 -24.7487 116.6726 Z"
        />
        <path
            d="M -123.7437 17.6777 A 75 75 45 0 1 -35.0725 4.6669 L -40.7647 12.9754 A 65 65 45 0 0 -116.6726 24.7487 Z"
        />
    </g>

    <g class:active={activeMid}>
        <path
            d="M -107.4802 33.9411 A 52 52 45 0 1 -48.26 23.8649 L -53.8462 32.2794 A 42 42 45 0 0 -100.4092 41.0122 Z"
        />
        <path
            d="M -33.9411 107.4802 A 52 52 45 0 0 -23.8649 48.26 L -32.2794 53.8462 A 42 42 45 0 1 -41.0122 100.4092 Z"
        />
    </g>

    <g class:active={activeClose}>
        <path
            d="M -50.2046 91.2168 A 29 29 45 0 0 -43.2042 61.4476 L -51.9723 67.3873 A 19 19 45 0 1 -57.2756 84.1457 Z"
        />
        <path
            d="M -91.2168 50.2046 A 29 29 45 0 1 -61.4476 43.2042 L -67.3873 51.9723 A 19 19 45 0 0 -84.1457 57.2756 Z"
        />
    </g>
</svg>

<!-- 
above is from the below + 45deg rotation

https://yqnn.github.io/svg-path-editor/

M 0 100
L -10 20 h -13
L 0 -5
L 23 20 h -13
Z

M 75 100
A 75 75 0 0 0 21.5 28.1
L 19.65 38
A 65 65 0 0 1 65 100
Z

M 52 100
A 52 52 0 0 0 17.25 51
L 15.25 60.9
A 42 42 0 0 1 42 100
Z

M 29 100
A 29 29 0 0 0 12.9 74
L 10.9 84.4
A 19 19 0 0 1 19 100
Z

M -75 100
A 75 75 0 0 1 -21.5 28.1
L -19.65 38
A 65 65 0 0 0 -65 100
Z

M -52 100
A 52 52 0 0 1 -17.25 51
L -15.25 60.9
A 42 42 0 0 0 -42 100
Z

M -29 100
A 29 29 0 0 1 -12.9 74
L -10.9 84.4
A 19 19 0 0 0 -19 100
Z

M 8 100
L 23 20

M 75 100
A 75 75 0 0 0 -75 100

M 65 100 
A 65 65 0 0 0 -65 100

M 52 100
A 52 52 0 0 0 -52 100

M 42 100 
A 42 42 0 0 0 -42 100

M 29 100
A 29 29 0 0 0 -29 100

M 19 100 
A 19 19 0 0 0 -19 100

 -->

<style lang="postcss">
    g > path {
        fill: hsl(var(--muted-foreground) / 40%);
    }

    g.active > path,
    .arrow {
        fill: hsl(var(--foreground));
    }
</style>
