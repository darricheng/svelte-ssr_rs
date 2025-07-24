<script>
  import svelteLogo from './assets/svelte.svg'
  import viteLogo from '/vite.svg'
  import Counter from './lib/Counter.svelte'

  // NOTE: For capturing props passed during server rendering.
  // See the network request response.
  // This causes FE to have errors though, because there's no
  // such props being passed when this code is run in the client.
  // let { some } = $props();
</script>

<svelte:head><title>Vite + Svelte + Rust SSR</title></svelte:head>
<main>
  <div>
    <a href="https://vitejs.dev" target="_blank" rel="noreferrer">
      <img src={viteLogo} class="logo" alt="Vite Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank" rel="noreferrer">
      <img src={svelteLogo} class="logo svelte" alt="Svelte Logo" />
    </a>
  </div>
  <h1>Vite + Svelte</h1>

  <div class="card">
    <Counter />
    <Counter initialVal={5} />
  </div>

  <p>
    Check out <a href="https://github.com/sveltejs/kit#readme" target="_blank" rel="noreferrer">SvelteKit</a>, the official Svelte app framework powered by Vite!
  </p>

  <p class="read-the-docs">
    Click on the Vite and Svelte logos to learn more
  </p>

  <!-- NOTE: these are to show ssr works -->
  <!-- {#each some.params_1 as lang} -->
  <!--   <p>{lang}</p> -->
  <!-- {/each} -->
  <!---->
  <!-- <p>{some.foo}</p> -->
  <!---->
  <!-- <ul> -->
  <!--   <li>{some.someObj.prop1}</li> -->
  <!--   <li>{some.someObj.prop2}</li> -->
  <!-- </ul> -->
</main>

<style>
  .logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
    transition: filter 300ms;
  }
  .logo:hover {
    filter: drop-shadow(0 0 2em #646cffaa);
  }
  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00aa);
  }
  .read-the-docs {
    color: #888;
  }
</style>
