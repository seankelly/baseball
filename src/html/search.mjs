function send_query() {
    let sql_query_el = document.getElementById('sql-query');
    const sql_query = sql_query_el.value;
    let url = new URL('sql', window.location.href);
    url.searchParams.set('query', sql_query);
    fetch(url)
        .then((response) => response.json())
        .then(show_query_results);
}

function show_query_results(response) {
    let table_el = document.getElementById('sql-results');
    if (!response.hasOwnProperty('columns') || !response.hasOwnProperty('rows')) {
        show_query_error(response);
        return;
    }
    let fragment = document.createDocumentFragment();
    let header = document.createElement('tr');
    fragment.appendChild(header);
    for (const column of response['columns']) {
        let name = document.createElement('th');
        name.innerText = column;
        header.appendChild(name);
    }

    for (const row of response['rows']) {
        let tr = document.createElement('tr');
        fragment.appendChild(tr);
        for (const cell of row) {
            let td = document.createElement('td');
            tr.appendChild(td);
            td.innerText = cell;
            if (typeof cell === "number") {
                td.classList.add("number");
            }
            else {
                td.classList.add("text");
            }
        }
    }

    table_el.replaceChildren(...[fragment]);
}

function show_query_error() {
}

function init() {
    let sql_submit = document.getElementById("submit-sql-query");
    sql_submit.addEventListener('click', send_query);
}

init();
