console.log("worker run")

addEventListener('message', (e) => {
    switch (e.data) {
        case 'ping':
            ping()
            break
        case 'download':
            download()
            break
        case 'upload':
            upload()
    }
})

function upload() {
    let i = 0
    let buf = new ArrayBuffer(1048576);
    let s = setInterval(() => {
        if (i === 10) {
            clearInterval(s)
        }
        i++
        let request = new XMLHttpRequest()
        request.open('POST', 'upload', false)
        request.onprogress = (e) => {
            console.log('onprogress', e)
        }

        request.onload = function () {
            if (request.status >= 200 && request.status < 400) {
                postMessage(['upload', Math.round(8000 / (Date.now() - start))])
                console.log("time: ", Date.now() - start)
            }
        }
        let start = Date.now()
        request.send(buf)
    }, 300)
}

function download() {
    let i = 0
    let s = setInterval(() => {
        if (i === 10) {
            clearInterval(s)
        }
        i++
        let request = new XMLHttpRequest()
        request.open('GET', 'download', false)
        request.onload = () => {
            if (request.status >= 200 && request.status < 400) {
                postMessage(['download', Math.round(8000 / (Date.now() - start))])
                console.log("time: ", Date.now() - start)
            }
        }
        let start = Date.now()
        request.send()
    }, 300)
}

function ping() {
    let i = 0
    let s = setInterval(() => {
        if (i === 10) {
            clearInterval(s)
        }
        i++
        let request = new XMLHttpRequest()
        request.open('GET', 'ping', false)
        request.onload = () => {
            if (request.status >= 200 && request.status < 400) {
                //let data = JSON.parse(this.response);
                postMessage(['ping', Date.now() - start])
                console.log("time: ", Date.now() - start)
            }
        }
        let start = Date.now()
        request.send()
    }, 300)

}