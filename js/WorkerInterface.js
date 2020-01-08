//interface to worker

export default class WorkerInterface extends Worker{

    constructor(script) {
        super(script);
        this.function_calls_n = 0;
        this.function_calls = {};
        this.to_call = [];
        this.onmessage = function(e) {
            if (e.data == "ready") {
                this.to_call.forEach(d => d());
                delete this.to_call;
            } else {
                let data = JSON.parse(e.data);
                if (data.end) {
                    delete this.function_calls[data.n];
                } else {
                    this.function_calls[data.n](data.data);
                }
            }
        }
        return new Proxy(this, {
            get(target, f) {
                if (f == "postMessage") {
                    return target.postMessage
                }

                return function(args,callback) {
                    let to_be_called = function() {
                        console.log(f+" called")
                        target.function_calls[target.function_calls_n] = callback
                        target.postMessage(JSON.stringify({
                            f,
                            n:target.function_calls_n,
                            args
                        }))
                        target.function_calls_n++;
                    }
                    if (target.to_call) {
                        target.to_call.push(to_be_called);
                    } else {
                        to_be_called();
                    }
                }
            },

            set(target, e, val) {
                if (e == "onmessage") {
                    target.onmessage = val
                    return true
                } else {
                    return false
                }
            }
        })
  }
}
