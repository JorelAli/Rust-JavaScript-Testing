extern crate web_view;

use web_view::*;
use std::fs;

fn main() {
    let size = (800, 550);
    let resizable = true;
    let debug = false;
    let titlebar_transparent = true;
    // let frontend_cb = |_webview: &mut _, _arg: &_, _userdata: &mut _| {
    // };
    let userdata = "hi";

    let html = format!(r#"
        <!DOCTYPE HTML>
        <html>
        <head>
          <meta charset="UTF-8">
          <title>Main</title>

          <!-- Styles -->
          <link rel="stylesheet" href="https://stackpath.bootstrapcdn.com/bootstrap/4.3.1/css/bootstrap.min.css" integrity="sha384-ggOyR0iXCbMQv3Xipma34MD+dH/1fQ784/j6cY/iJTQUOhcWr7x9JvoRxT2MZw1T" crossorigin="anonymous">

          <style>{css}</style>

          <!-- Elm -->
          <script>{elm}</script>
        </head>
        
        <body>
          <div>
            <button onclick="external.invoke('opendialog')">Open save dialog</button>
            <br>
            <button onclick="external.invoke('sysalert')">Open alert</button>
            <br>
            <textarea id="jsMsg" rows="200" cols="100"></textarea>
          </div>
          <script>
            function myJavaScriptFunction(text) {{
                //console.log("hi")
                document.getElementById('jsMsg').innerHTML = '' + text;
                //alert("hello");
            }}
          </script>
        </body>
        </html>
    "#,
    css = include_str!("../www/style.css"),
    elm = include_str!("../www/main.js"));

    /*
    run(title, content, size, resizable?, debug?, titlebar_transparent?, init_cb, ext_cb, user_data)
    */
    run(
        "hello",
        Content::Html(html),
        Some(size),
        resizable,
        debug,
        titlebar_transparent,
        move |mut webview| {
            webview.set_background_color(0.11, 0.12, 0.13, 1.0);
        },
        move |webview, arg, _userdata| {

          match arg {
            
            // "sysalert" => {
            //   webview.dialog(Dialog::Alert(Alert::Info), "hi", Some("hi there"));
            // }
            "opendialog" => {
              let some_str = /*diag(webview);*/webview.dialog(Dialog::OpenFile, "hello", Some("hi there"));
              let lines = fs::read_to_string(some_str).expect("Can't read file.");
              let lines = lines.replace("\"", "\\\"");
              let lines = lines.replace("\r\n", "\\n");
              let lines = lines.replace("\n", "\\n");

              let exec = &format!("myJavaScriptFunction(\"{}\")", lines);

              // println!("{}", some_str);
              // println!("{}", lines);
              // println!("{}", exec);
              println!("{}", userdata);
              //Using this \"{}\" method is not "perfect". It still doesn't escape strings properly
              webview.eval(exec);
            }
            "sysalert" => {
              //test(webview);
              webview.dialog(Dialog::Alert(Alert::Info), "hi", Some("hi there"));
              println!("hi");
            }
            _ => unimplemented!()
          }

          // webview.eval("hi");

          // webview.dialog("hi", "hi", "hi");
        },
        userdata
    );
}

// fn diag<'a, T>(webview: &mut WebView<'a, T>) -> std::string::String {
//   return webview.dialog(Dialog::OpenFile, "hello", Some("hi there"));
// }

// fn test<'a, T>(webview: &mut WebView<'a, T>) {
//   webview.dialog(Dialog::Alert(Alert::Info), "hi", Some("hi there"));
// }