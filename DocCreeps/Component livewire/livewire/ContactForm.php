<?php
namespace App\Livewire;

use Livewire\Component;

class ContactForm extends Component
{
    public $name;
    public $email;
    public $message;

    protected $rules = [
        'email' => 'required|email',
        'message' => 'required',
    ];

    public function sendMessage()
    {
        $this->validate();

        // Ici, tu peux ajouter le code pour envoyer le message par email ou le traiter selon tes besoins

        session()->flash('message', 'Message envoyé avec succès!');

        $this->reset();
    }

    public function render()
    {
        return view('livewire.contact-form');
    }
}