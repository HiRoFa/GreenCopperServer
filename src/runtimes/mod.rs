// since this will not be reuasable we'll just impl this as a lazy)static vec of EsRuntimes

use quickjs_runtime::esruntime::EsRuntime;

async fn with_runtime<C: FnOnce(&EsRuntime)>(consumer: C){
    //
}

async fn call_method() {
    with_runtime(|rt| {
        //rt.call_function()
    }).await
}