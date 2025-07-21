<script>
  let props = $props();

  let { initialVal = 0 } = props;

  let count = $state(initialVal);
  const increment = () => {
    count += 1
  }
</script>

<button onclick={increment}>
  count is {count}
</button>
