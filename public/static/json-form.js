// s1 ∩ s2
function intersection(s1, s2) {
    return new Set([...s1].filter((x) => s2.has(x)));
}

// s1 \ s2
function difference(s1, s2) {
    return new Set([...s1].filter((x) => !s2.has(x)));
}

async function submitJSON(path, form_id) {
    const form = document.getElementById(form_id);
    if (!form.checkValidity()) {
        return;
    }
    const fd = new FormData(form);
    let keys = new Set(fd.keys());
    let multi_sel = new Set(
        Array.from(form.querySelectorAll('select[multiple]')).map((f) => f.name)
    );
    // convert form to JSON
    let data = {};
    for (const key of intersection(keys, multi_sel)) {
        data[key] = fd.getAll(key);
    }
    for (const key of difference(keys, multi_sel)) {
        data[key] = fd.get(key);
    }
    await fetch(path, {
        method: 'POST',
        body: JSON.stringify(data),
        headers: {
            'content-type': 'application/json',
        },
    });
    form.reset();
    location.reload();
}
