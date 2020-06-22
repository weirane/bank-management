async function query(path, form_id) {
    const form = document.getElementById(form_id);
    if (!form.checkValidity()) {
        return;
    }
    const data = new URLSearchParams();
    const fd = new FormData(form);
    for (const [key, val] of fd.entries()) {
        data.append(key, val);
    }
    const res = await fetch(path, {
        method: 'POST',
        body: data,
    });
    const json = await res.json();
    console.log(json);
    form.reset();
}
