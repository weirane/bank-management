const acctype = document.querySelector('#query-account select');
const save = document.getElementById('type-save');
const check = document.getElementById('type-check');
for (const types of acctype.getElementsByTagName('option')) {
    types.addEventListener('click', (_) => {
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
    });
}
