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
    let userdata = ();

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
            <textarea id="jsMsg" rows="200" cols="100"></textarea>
          </div>
          <script>
            function myJavaScriptFunction(text) {{
                console.log("hi")
                document.getElementById('jsMsg').innerHTML = '' + text;
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
        move |webview, arg, userdata| {

          match arg {
            "opendialog" => {
              let some_str = webview.dialog(Dialog::OpenFile, "hello", Some("hi there"));
              let lines = fs::read_to_string(some_str).expect("Can't read file.");
              let lines = lines.replace("\"", "\\\"");
              let lines = lines.replace("\r\n", "\\n");

              let exec = &format!("myJavaScriptFunction(\"{}\")", lines);

              // println!("{}", some_str);
              // println!("{}", lines);
              println!("{}", exec);
              //Using this \"{}\" method is not "perfect". It still doesn't escape strings properly
              webview.eval(exec);
            }
            _ => unimplemented!()
          }

          // webview.eval("hi");

          // webview.dialog("hi", "hi", "hi");
        },
        userdata
    );
}
