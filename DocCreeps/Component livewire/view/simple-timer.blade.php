<div class="mt-8 text-center">
    <h2 class="text-3xl mb-4">{{ gmdate("H:i:s", $seconds) }}</h2>

    <button wire:click="startTimer" wire:loading.attr="disabled" wire:target="startTimer" class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded mr-2">Démarrer</button>
    <button wire:click="stopTimer" wire:loading.attr="disabled" wire:target="stopTimer" class="bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded mr-2">Arrêter</button>
    <button wire:click="resetTimer" wire:loading.attr="disabled" wire:target="resetTimer" class="bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded">Réinitialiser</button>
</div>


<script>
   document.addEventListener('livewire:load', function () {
    window.livewire.on('timerStarted', function () {
        setInterval(function () {
            window.livewire.emit('updateSeconds');
        }, 1000);
    });

    window.livewire.on('timerStopped', function () {
        clearInterval();
    });
});

</script>
