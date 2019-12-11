export default class WorkerInterface extends Worker{

    constructor(script) {
        super(script);
        this.function_calls_n = 0;
        this.function_calls = {};
        this.onmessage = function(e) {
            console.log(e.data)
            let data = JSON.parse(e.data);

            if (data.end) {
                delete this.function_calls[data.n];
            } else {
                this.function_calls[data.n](data.data);
            }
        }
        return new Proxy(this, {
            get(target, f) {
                if (f == "postMessage") {
                    return target.postMessage
                }

                return function(args,callback) {
                    console.log(f+" called")
                    target.function_calls[target.function_calls_n] = callback
                    target.postMessage(JSON.stringify({
                        f,
                        n:target.function_calls_n,
                        args
                    }))
                    target.function_calls_n
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
