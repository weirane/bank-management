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

const constructOption = (json, datas, interval, title) => {
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
                data: datas[b]
                    .reverse()
                    .map(([d, n]) => [d.substring(0, d.length - (interval === 'year' ? 6 : 3)), n]),
            };
        }),
    };
};

/** convert `json` to the data for echarts */
const constructDatas = (json, interval, extract) => {
    const datas = json.banks.reduce((obj, cur) => {
        obj[cur] = new Array();
        return obj;
    }, {});
    let later = false;
    for (const kv of json.datas) {
        const [date, stats] = Object.entries(kv)[0];
        if (later) {
            const objdate = new Date(date);
            if (interval === 'year') {
                if (objdate.getUTCMonth() !== 0) {
                    continue;
                }
            } else if (interval === 'season') {
                if (objdate.getUTCMonth() % 3 !== 0) {
                    continue;
                }
            }
        } else {
            later = true;
        }
        for (const b of json.banks) {
            const d = findBankObj(stats, b);
            datas[b].push([date, extract(d)]);
        }
    }
    return datas;
};

function drawCustomer(eid, json, interval) {
    const div = document.getElementById(eid);
    const datas = constructDatas(json, interval, (d) => d.total_customer);
    const chart = echarts.init(div);
    chart.setOption(constructOption(json, datas, interval, '客户统计'));
}

function drawBusiness(eid, json, interval) {
    const div = document.getElementById(eid);
    const datas = constructDatas(json, interval, (d) =>
        TYPE == 'save' ? d.total_balance : d.total_loanpay
    );
    const chart = echarts.init(div);
    chart.setOption(constructOption(json, datas, interval, '金额统计'));
}

const intervalSel = document.getElementById('interval');
for (const o of intervalSel.getElementsByTagName('option')) {
    o.addEventListener('click', (_) => {
        if (renderWithInterval) {
            renderWithInterval(intervalSel.value);
        }
    });
}

let renderWithInterval;

(async () => {
    const json = await getData(TYPE);
    renderWithInterval = (interval) => {
        drawCustomer('customer-stat', json, interval);
        drawBusiness('business-stat', json, interval);
    };
    renderWithInterval('month');
})();
