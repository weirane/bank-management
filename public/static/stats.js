const TYPE = new URLSearchParams(new URL(document.currentScript.src).search).get('type');
if (TYPE !== 'save' && TYPE !== 'check') {
    throw new Error(`invalid type ${TYPE}`);
}

/** find the stats for `bank` in `stats` */
const findBankObj = (stats, bank) => {
    for (const s of stats) {
        if (s.bank == bank) {
            return s;
        }
    }
    throw new Error(`cannot find bank ${bank}`);
};

const getData = async () => {
    const res = await fetch(`/stats/${TYPE}`, {
        method: 'post',
    });
    const json = await res.json();
    return json;
};

const constructOption = (json, datas, title) => {
    return {
        title: { text: title },
        tooltip: { trigger: 'axis' },
        legend: {
            data: json.banks.map((x) => {
                return { name: x };
            }),
        },
        dataZoom: {
            show: true,
            realtime: true,
            start: json.datas.length > 20 ? 50 : 0,
            end: 100,
        },
        xAxis: { name: '日期', type: 'category' },
        yAxis: { type: 'value' },
        series: json.banks.map((b) => {
            return {
                name: b,
                type: 'line',
                data: datas[b].reverse(),
            };
        }),
    };
};

/** convert `json` to the data for echarts */
const constructDatas = (json, extract) => {
    const datas = json.banks.reduce((obj, cur) => {
        obj[cur] = new Array();
        return obj;
    }, {});
    for (const kv of json.datas) {
        const [date, stats] = Object.entries(kv)[0];
        for (const b of json.banks) {
            const d = findBankObj(stats, b);
            datas[b].push([date, extract(d)]);
        }
    }
    return datas;
};

function drawCustomer(eid, json) {
    const div = document.getElementById(eid);
    const datas = constructDatas(json, (d) => d.total_customer);
    const chart = echarts.init(div);
    chart.setOption(constructOption(json, datas, '客户统计'));
}

function drawBusiness(eid, json) {
    const div = document.getElementById(eid);
    const datas = constructDatas(json, (d) => (TYPE == 'save' ? d.total_balance : d.total_loanpay));
    const chart = echarts.init(div);
    chart.setOption(constructOption(json, datas, '金额统计'));
}

(async () => {
    const json = await getData(TYPE);
    drawCustomer('customer-stat', json);
    drawBusiness('business-stat', json);
})();
