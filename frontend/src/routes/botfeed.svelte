<script lang="ts">
  // Botfeed component logic can be added here in the future
  import Feeditem from "./feeditem.svelte";
  import type { FeedItem } from "../lib/types";
  import {faker} from "@faker-js/faker";
  import { flip } from "svelte/animate";
  const BOTFEED_MAX_ITEMS = 50;
  let testing = true;

  const createPlaceholderItem = (): FeedItem => {
    return {
      author_id: faker.number.int().toString(),
      author_name: faker.internet.username(),
      command_name: faker.hacker.verb() + " " + faker.hacker.noun(),
      command_output: faker.lorem.sentence()
    };
  };

  const createPlaceholderItemsMultiple = (count: number): FeedItem[] => {
    const items: FeedItem[] = [];
    for (let i = 0; i < count; i++) {
      items.push(createPlaceholderItem());
    }
    return items;
  };

  let botfeedItems: FeedItem[] = [];

  function updateBotFeed(newItem: FeedItem) {
    // update botfeedItems with new entries prepended and trim the list length to BOTFEED_MAX_ITEMS
    botfeedItems = [newItem, ...botfeedItems].slice(0, BOTFEED_MAX_ITEMS);
  }

  function clearBotFeed() {
    botfeedItems = [];
  }

  function addOnePlaceholderItemToFeed() {
    const newItem = createPlaceholderItem();
    updateBotFeed(newItem);
  }

  function addFivePlaceholderItemToFeed() {
    const fiveNewItems = createPlaceholderItemsMultiple(5);
    fiveNewItems.forEach(item => updateBotFeed(item));
  }

</script>

<div class="botfeed-container">
  <h1 class="botfeed-header">Bot Feed</h1>
  {#if testing}
  <div class="botfeed-testing-note">
    <strong>Testing Mode Enabled:</strong> Use the buttons below to add placeholder items to the bot feed.
    <button class="botfeed-button-test" on:click={addOnePlaceholderItemToFeed}>
      Add One Placeholder Item
    </button>
    <button class="botfeed-button-test" on:click={addFivePlaceholderItemToFeed}>
      Add Five Placeholder Items
    </button>
    <button class="botfeed-button-test" on:click={clearBotFeed}>
      Clear Bot Feed
    </button>
  </div>
  {/if}
  <div class="botfeed-items">
    {#each botfeedItems as item (item.author_id)}
    <div class="botfeed-item" animate:flip={{ duration: 200 }}>
      <Feeditem 
        author_id={item.author_id} 
        author_name={item.author_name} 
        command_name={item.command_name} 
        command_output={item.command_output}
      />
    </div>
    {/each}
  </div>
</div>