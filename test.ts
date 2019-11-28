import {Connection} from './pkg/hdbconnect_wasm'

function test(){

    try{
        let con  = new Connection("test", 30515, "test", "test");

        console.log(con.query("SELECT * FROM DUMMY"))

    }catch(err){
        console.log("Error:"+err.message)
    }
}

test();
