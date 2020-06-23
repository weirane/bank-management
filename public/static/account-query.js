async function query(path, form_id) {
    // extract form data
    const form = document.getElementById(form_id);
    if (!form.checkValidity()) {
        return;
    }
    const data = new URLSearchParams();
    const fd = new FormData(form);
    for (const [key, val] of fd.entries()) {
        data.append(key, val);
    }

    // send request
    const res = await fetch(path, {
        method: 'POST',
        body: data,
    });
    const customers = await res.json();

    // present result in table
    const tbody = document.querySelector('#result tbody');
    tbody.innerHTML = '';
    if (customers.length == 0) {
        // no result
        result.classList.add('empty');
        form.reset();
        return;
    }
    result.classList.remove('empty');
    for (const cus of customers) {
        const tr = document.createElement('tr');
        for (const key of ['id', 'type', 'bank', 'open_date', 'balance']) {
            const el = document.createElement('td');
            el.innerText = cus[key];
            tr.appendChild(el);
        }
        const el = document.createElement('td');
        if (cus.type == '储蓄账户') {
            el.innerText = `货币类型：${cus.currency}，利率：${cus.interest_rate}`;
        } else if (cus.type == '支票账户') {
            el.innerText = `透支额：${cus.credit}`;
        }
        tr.appendChild(el);
        tbody.appendChild(tr);
    }

    // reset
    form.reset();
    save.classList.add('hide');
    check.classList.add('hide');
}

const acctype = document.querySelector('#query-account select');
const save = document.getElementById('type-save');
const check = document.getElementById('type-check');
const checkAccountType = () => {
    if (acctype.value === '') {
        save.classList.add('hide');
        check.classList.add('hide');
    } else if (acctype.value === '0') {
        save.classList.remove('hide');
        check.classList.add('hide');
    } else if (acctype.value === '1') {
        save.classList.add('hide');
        check.classList.remove('hide');
    }
};

for (const types of acctype.getElementsByTagName('option')) {
    types.addEventListener('click', checkAccountType);
}

checkAccountType();
