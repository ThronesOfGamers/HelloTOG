<div x-data="{ currentImage: 0 }">
    <img :src="'/path/to/images/' + images[currentImage]" alt="Image" x-bind:key="currentImage">
    <button x-on:click="currentImage = (currentImage + 1) % images.length">Suivant</button>
    <button x-on:click="currentImage = (currentImage - 1 + images.length) % images.length">Précédent</button>
</div>
