<div class="max-w-md mx-auto bg-white rounded-md overflow-hidden shadow-md p-6">
    <form wire:submit.prevent="sendMessage">
        @if (session()->has('message'))
            <div class="mb-4 text-green-600">{{ session('message') }}</div>
        @endif

        <div class="mb-4">
            <label class="block text-gray-700 text-sm font-bold mb-2">Nom et Prénom (facultatif)</label>
            <input class="border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" type="text" wire:model="name">
        </div>

        <div class="mb-4">
            <label class="block text-gray-700 text-sm font-bold mb-2">Email (obligatoire)</label>
            <input class="border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" type="email" wire:model="email">
            @error('email') <span class="text-red-500">{{ $message }}</span> @enderror
        </div>

        <div class="mb-4">
            <label class="block text-gray-700 text-sm font-bold mb-2">Message (obligatoire)</label>
            <textarea class="border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" wire:model="message"></textarea>
            @error('message') <span class="text-red-500">{{ $message }}</span> @enderror
        </div>

        <div class="text-center">
            <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline" type="submit">Envoyer</button>
        </div>
    </form>
</div>