function ajouterTache() {
    var tache = document.getElementById("task").value;
    var listeTaches = document.getElementById("listeTaches");
    var nouvelleTache = document.createElement("li");
    nouvelleTache.className = "list-group-item d-flex justify-content-between align-items-center";
    nouvelleTache.innerHTML = `
        ${tache}
        <div>
            <button class="btn btn-success btn-sm" onclick="marquerTerminee(this)">Terminée</button>
            <button class="btn btn-danger btn-sm ml-2" onclick="supprimerTache(this)">Supprimer</button>
        </div>
    `;
    listeTaches.appendChild(nouvelleTache);
    document.getElementById("task").value = "";
}

function marquerTerminee(bouton) {
    bouton.parentElement.parentElement.classList.toggle("list-group-item-success");
}

function supprimerTache(bouton) {
    bouton.parentElement.parentElement.remove();
}
