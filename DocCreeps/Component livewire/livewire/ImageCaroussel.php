use Livewire\Component;

class ImageCarousel extends Component
{
    public $images = [
        'image1.jpg',
        'image2.jpg',
        'image3.jpg',
        // Ajoute d'autres noms de fichiers d'images ici
    ];

    public $currentImage = 0;

    public function nextImage()
    {
        $this->currentImage = ($this->currentImage + 1) % count($this->images);
    }

    public function prevImage()
    {
        $this->currentImage = ($this->currentImage - 1 + count($this->images)) % count($this->images);
    }

    public function render()
    {
        return view('livewire.image-carousel');
    }
}
