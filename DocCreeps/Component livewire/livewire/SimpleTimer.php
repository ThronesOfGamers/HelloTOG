<?php
namespace App\Livewire;
use Livewire\Component;

class SimpleTimer extends Component
{
    public $seconds = 0;
    public $isRunning = false;

    public function startTimer()
    {
        $this->isRunning = true;
    }

    public function stopTimer()
    {
        $this->isRunning = false;
    }

    public function resetTimer()
    {
        $this->seconds = 0;
        $this->isRunning = false;
    }

    public function updatedIsRunning($value)
    {
        if ($value) {
            $this->emit('timerStarted');
        } else {
            $this->emit('timerStopped');
        }
    }

    public function updatedSeconds()
    {
        $this->emit('timerUpdated', $this->seconds);
    }

    public function render()
    {
        return view('livewire.simple-timer');
    }
}


