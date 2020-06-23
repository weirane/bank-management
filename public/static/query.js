async function query(path, form_id, fields) {
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
    const customers = await res.json();
    const tbody = document.querySelector('#result tbody');
    tbody.innerHTML = '';
    if (customers.length == 0) {
        result.classList.add('empty');
        form.reset();
        return;
    }
    result.classList.remove('empty');
    for (const cus of customers) {
        const tr = document.createElement('tr');
        for (const key of fields) {
            const el = document.createElement('td');
            el.innerText = cus[key].trim();
            tr.appendChild(el);
        }
        tbody.appendChild(tr);
    }
    form.reset();
}
