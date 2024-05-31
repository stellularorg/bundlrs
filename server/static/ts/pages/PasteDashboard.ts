const error: HTMLElement = document.getElementById("error")!;
const success: HTMLElement = document.getElementById("success")!;

const reclaim_form: HTMLFormElement | null = document.getElementById(
    "reclaim_paste",
) as HTMLFormElement | null;

if (reclaim_form) {
    // reclaim paste
    reclaim_form.addEventListener("submit", async (e) => {
        e.preventDefault();
        const res = await fetch(`/api/claim/${reclaim_form.custom_url.value}`, {
            method: "POST",
        });

        const json = await res.json();

        if (json.success === false) {
            error.style.display = "block";
            error.innerHTML = `<div class="mdnote-title">${json.message}</div>`;
        } else {
            success.style.display = "block";
            success.innerHTML = `<div class="mdnote-title">${json.message}</div>`;
        }
    });
}
