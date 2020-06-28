const account_id = document.getElementById('account-id');
const customers = document.getElementById('acc-customers');

function changeCustomer() {
    const val = account_id.value;
    const cus = choice.getValue(true);
    fetch('/account/customers', {
        method: 'POST',
        body: JSON.stringify({
            account: val,
            customers: cus,
        }),
        headers: {
            'content-type': 'application/json',
        },
    }).then((_) => {
        window.location.reload();
    });
}

let choice;

(async () => {
    choice = new Choices(customers, { position: 'bottom', removeItemButton: true });
    const res = await fetch('/account/customers');
    const json = await res.json();
    function updateCustomer(_) {
        const val = account_id.value;
        choice.destroy();
        choice = new Choices(customers, { position: 'bottom', removeItemButton: true });
        choice.setChoiceByValue(json[val]);
    }
    account_id.addEventListener('change', updateCustomer);
    updateCustomer();
})();
