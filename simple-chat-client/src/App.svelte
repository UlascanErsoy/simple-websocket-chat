<script lang="ts">
    import { writable } from 'svelte/store';
    let username = "ulascan";
    let hostname = "127.0.0.1:3001";
    let message = "";
    let socket = null;

    let messages = writable([]);
     
    let handleConnection = () => {
        socket = new WebSocket(`ws:\\\\${hostname}?u=${username}`);

        socket.addEventListener("message", (event) => {
            console.log("Message from the server:", event.data);
            $messages = [...$messages, JSON.parse(event.data)];
        });
    }

    let sendMessage = () => {
        if (socket != null) {
            socket.send(message);
        }
    };
</script>

<main>
<nav class="bg-white border-gray-200 dark:bg-gray-900">
  <div class="max-w-screen-xl flex flex-wrap items-center justify-between mx-auto p-4">
  <input bind:value={username} type="text" id="username" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Username" required>
  <input bind:value={hostname} type="text" id="hostname" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="ws://..." required>
  <button on:click={handleConnection} type="button" class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 mr-2 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800">Connect</button>
</div>
</nav>
  <div class="max-w-screen-xl flex flex-wrap items-center justify-between mx-auto p-4">
  <input bind:value={message} type="text" id="message" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="type your message" required>
  <button on:click={sendMessage} type="button" class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 mr-2 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800">Send Message</button>
</div>
{#each $messages as msg}
    <div class="block max-w-sm p-6 bg-white border border-gray-200 rounded-lg shadow hover:bg-gray-100 dark:bg-gray-800 dark:border-gray-700 dark:hover:bg-gray-700">
        <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">{msg.user.username}</h5>
        <p class="font-normal text-gray-700 dark:text-gray-400">{msg.msg}</p>
    </div>
{/each}
</main>

<style>
</style>
