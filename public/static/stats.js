async function getData(type) {
    const res = await fetch(`/stats/${type}`, {
        method: 'post',
    });
    const json = await res.json();
    return json;
}

const div = document.getElementById('main');

getData(div.dataset.type).then((data) => {
    console.log(data);
});
