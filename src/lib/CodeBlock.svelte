<script>
    import { Highlight } from 'svelte-highlight';
    import javascript from 'svelte-highlight/languages/javascript';
    import 'svelte-highlight/styles/github.css';
  
    export let code = `const greet = () => { console.log('Hello, world!'); };`;
  </script>
  
  <Highlight language={javascript} {code} />
  