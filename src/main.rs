slint::slint!{
    import { Button, VerticalBox } from "std-widgets.slint";

    component Button {
        in property <string> text;
        Rectangle {
            background: ta.pressed ? red : #1d78d3;
            border-radius: 5px;
            border-width: 2px;
            border-color: self.background.darker(20%);
            ta := TouchArea {}
        }
        Text { 
            text: root.text;
            color: ta.pressed ? white : #000;
        }
    }

    export component App inherits Window {
        in property <int> counter: 1;
        GridLayout {
            spacing: 10px;
            padding: 20px;
            Row {
                Text { text: "Hello world " + counter; colspan: 3;}
            }
            
            Row {
                Button { text: "1"; }
                Button { text: "2"; }
                Button { text: "3"; }
            }
            Row {
                Button { text: "4"; }
                Button { text: "5"; }
                Button { text: "6"; }
            }
            Row {
                Button { text: "7"; }
                Button { text: "8"; }
                Button { text: "9"; }
            }
            Row {
                Button { text: "0"; colspan: 2;}
            }
        }
        
    }
}

fn main() {
    let app = App::new().unwrap();
    let weak = app.as_weak();
    // app.on_clicked(move || {
    //     let app = weak.upgrade().unwrap();
    //     app.set_counter(app.get_counter() + 1);
    // });
    app.run().unwrap();
    println!("Hello, world!");
}
