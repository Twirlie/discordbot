<script lang="ts">
  import type { FeedItem } from "../lib/types";
  import { slide } from "svelte/transition";


  export let author_id: FeedItem["author_id"];
  export let author_name: FeedItem["author_name"];
  export let command_name: FeedItem["command_name"];
  export let command_output: FeedItem["command_output"];
</script>

<div class="feeditem" in:slide={{ duration: 200, axis: 'x'}}>
  <h2 class="feeditem-command">{command_name}</h2>
  <p class="feeditem-author">Executed by: {author_name} (ID: {author_id})</p>
  <pre class="feeditem-output">{command_output}</pre>
</div>