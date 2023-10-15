<div class="flex items-center space-x-4">
    <button wire:click="decrement" class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">-</button>
    <span class="text-2xl font-bold">{{ $count }}</span>
    <button wire:click="increment" class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">+</button>
</div>
