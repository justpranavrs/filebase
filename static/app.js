// Filters visible folder and file cards based on the search query.
function filterItems(query) {
  const q = (query || "").toLowerCase().trim();
  for (const card of document.querySelectorAll(".grid .card")) {
    const title = card.querySelector(".card-title");
    card.hidden = !!(q && title && !title.textContent.toLowerCase().includes(q));
  }
}

// Cached DOM references for the delete confirmation modal.
let deleteModal = null;

function getDeleteModal() {
  if (!deleteModal) {
    const dialog = document.getElementById("delete-dialog");
    if (!dialog) return null;
    deleteModal = {
      dialog,
      name: document.getElementById("delete-item-name"),
      actionBtn: document.getElementById("delete-action-btn"),
    };
  }
  return deleteModal;
}

// Opens the shared delete confirmation modal.
function openDeleteModal(btn) {
  const name = btn.dataset.name?.trim();
  const url = btn.dataset.url;
  const target = btn.dataset.target;

  if (!name || !url || !target) {
    return;
  }

  const modal = getDeleteModal();
  if (!modal || !modal.dialog || !modal.name || !modal.actionBtn) {
    return;
  }

  btn.closest("dialog")?.close();

  modal.name.textContent = name;
  modal.actionBtn.onclick = function () {
    modal.dialog.close();
    htmx.ajax("DELETE", url, {
      target: target,
      swap: "outerHTML",
    });
  };

  modal.dialog.showModal();
}
