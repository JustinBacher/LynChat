<script>
    export let text;
    export let timestamp = new Date();

    // Format the timestamp
    function formatTime(date) {
        return date.toLocaleTimeString([], {
            hour: "2-digit",
            minute: "2-digit",
        });
    }
</script>

<div class="flex flex-col items-end mb-4">
    <div class="user-message">
        <p class="m-0">{text}</p>
    </div>
    <span class="text-xs text-gray-500 mt-1">{formatTime(timestamp)}</span>
</div>
