<script lang="ts">
  // Botfeed component logic can be added here in the future
  import Feeditem from "./feeditem.svelte";
  import type { FeedItem } from "../lib/types";
  import {faker} from "@faker-js/faker";
  import { flip } from "svelte/animate";
  import { onMount } from "svelte";
  
  const BOTFEED_MAX_ITEMS = 50;
  let testing = true; // set to true to enable testing buttons
  let ws: WebSocket | null = null;
  let wsConnected = false;

  
  /**
   * Creates a placeholder FeedItem using faker library
   */
  const createPlaceholderItem = (): FeedItem => {
    return {
      item_uuid: faker.string.uuid(),
      timestamp: new Date().toISOString(),
      author_id: faker.number.int().toString(),
      author_name: faker.internet.username(),
      command_name: faker.hacker.verb() + " " + faker.hacker.noun(),
      command_output: faker.lorem.sentence(30),
      test_item: true
    };
  };

  /**
   * Creates multiple placeholder FeedItems
   */
  const createPlaceholderItemsMultiple = (count: number): FeedItem[] => {
    const items: FeedItem[] = [];
    for (let i = 0; i < count; i++) {
      items.push(createPlaceholderItem());
    }
    return items;
  };

  /**
   * Array to hold FeedItems for the bot feed
   */
  let botfeedItems: FeedItem[] = [];


  /**
   * Updates the bot feed with a new FeedItem
   * @param newItem
   */
  function updateBotFeed(newItem: FeedItem) {
    // update botfeedItems with new entries prepended and trim the list length to BOTFEED_MAX_ITEMS
    botfeedItems = [newItem, ...botfeedItems].slice(0, BOTFEED_MAX_ITEMS);
  }
  /**
   * Clears all items from the bot feed
   */
  function clearBotFeed() {
    botfeedItems = [];
  }
  /**
   * Adds one placeholder FeedItem to the bot feed
   */
  function addOnePlaceholderItemToFeed() {
    const newItem = createPlaceholderItem();
    updateBotFeed(newItem);
  }
  /*
  * Adds five placeholder FeedItems to the bot feed
  */
  function addFivePlaceholderItemToFeed() {
    const fiveNewItems = createPlaceholderItemsMultiple(5);
    fiveNewItems.forEach(item => updateBotFeed(item));
  }
  /*
  * Handles incoming real FeedItems from the backend (placeholder for now)
  */
  function handleNewFeeditem(item: FeedItem) {
    // this function is called whenever a new feed item is received over the websocket
    updateBotFeed(item);
  }

  /**
   * Connects to the WebSocket server at /ws/feed
   */
  function connectWebSocket() {
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    const wsUrl = `${protocol}//${window.location.host}/ws/feed`;
    
    ws = new WebSocket(wsUrl);
    
    ws.onopen = () => {
      console.log('WebSocket connected');
      wsConnected = true;
    };
    
    ws.onmessage = (event) => {
      try {
        const feedItem = JSON.parse(event.data) as FeedItem;
        if (feedItem.item_uuid === undefined) {
          console.error('Received FeedItem without item_uuid:', feedItem);
          return;
        }
        handleNewFeeditem(feedItem);
      } catch (error) {
        console.error('Failed to parse WebSocket message:', error);
      }
    };
    
    ws.onerror = (error) => {
      console.error('WebSocket error:', error);
      wsConnected = false;
    };
    
    ws.onclose = () => {
      console.log('WebSocket disconnected');
      wsConnected = false;
      // Attempt to reconnect after 3 seconds
      setTimeout(connectWebSocket, 3000);
    };
  }

  /**
   * Initialize WebSocket connection on component mount
   */
  onMount(() => {
    connectWebSocket();
    
    return () => {
      if (ws) {
        ws.close();
      }
    };
  });

</script>

<div class="botfeed-container">
  <h1 class="botfeed-header">Bot Feed</h1>
  {#if testing}
  <div class="botfeed-testing-note">
    <svg xmlns="http://www.w3.org/2000/svg"fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6 notification-icon">
      <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v3.75m9-.75a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9 3.75h.008v.008H12v-.008Z" />
    </svg>
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
    {#each botfeedItems as item (item.item_uuid)}
    <div class="botfeed-item" animate:flip={{ duration: 200 }}>
      <Feeditem 
        item_uuid={item.item_uuid}
        timestamp={item.timestamp}
        author_id={item.author_id} 
        author_name={item.author_name} 
        command_name={item.command_name} 
        command_output={item.command_output}
        test_item={item.test_item}
      />
    </div>
    {/each}
  </div>
</div>