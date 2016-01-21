extern crate glib;
extern crate gtk;

use gtk::traits::*;
use gtk::signal::Inhibit;

fn main() {
  gtk::init();
  let builder = gtk::widgets::Builder::new_from_file("ui/nonsense.glade").unwrap();
  unsafe {
    let window : gtk::Window = builder.get_object("window1").unwrap();
    window.set_title("GTK POS");
    window.set_default_size(800,600);
    window.set_window_position(gtk::WindowPosition::Center);

    // let order_grid : gtk::Grid = builder.get_object("order-grid").unwrap();
    // let button = gtk::Button.new_with_label("Spicy Meatballs").unwrap();
    // order_grid.pack_start(button);

    let list_tree : gtk::TreeView = builder.get_object("treeview1").unwrap();

    let column_types = [glib::Type::String, glib::Type::String];
    let list_store = gtk::ListStore::new(&column_types).unwrap();
    let list_model = list_store.get_model().unwrap();

    list_tree.set_model(&list_model);
    list_tree.set_headers_visible(true);

    let column = gtk::TreeViewColumn::new().unwrap();
    let price = gtk::TreeViewColumn::new().unwrap();
    let cell = gtk::CellRendererText::new().unwrap();
    let price_cell = gtk::CellRendererText::new().unwrap();

    let food_button : gtk::Button = builder.get_object("button1").unwrap();
    let food_button2 : gtk::Button = builder.get_object("button2").unwrap();
    let handler = move |_| {
      let price = 10.12;
      let price_str: String = format!("${}", price);
      let iter = list_store.append();
      list_store.set_string(&iter, 0, "Root Beer");
      list_store.set_string(&iter, 1, &price_str);
    };
    food_button.connect_clicked(handler);
    // food_button2.connect_clicked(handler);

    column.pack_start(&cell, true);
    price.pack_start(&price_cell, true);
    column.set_title("Item");
    price.set_title("Price");
    column.set_expand(true);
    column.add_attribute(&cell, "text", 0);
    column.add_attribute(&price_cell, "text", 1);
    list_tree.append_column(&column);
    list_tree.append_column(&price);

    // let list_selection = list_tree.get_selection().unwrap();
    // list_selection.connect_changed(|tree_selection| {
    //     let (list_model, iter) = tree_selection.get_selected().unwrap();
    //     if let Some(path) = list_model.get_path(&iter) {
    //         println!("selected row {}", path.to_string().unwrap());
    //     }
    // });

    // for _ in 0..10 {
    //     let iter = list_store.append();
    //     list_store.set_string(&iter, 0, "I'm in a list");
    //     list_store.set_string(&iter, 1, "$ 0.99");

    //     // select this row as a test

    //     if let Some(path) = list_model.get_path(&iter) {
    //         list_selection.select_path(&path);
    //     }
    // }

    window.show_all();
    window.connect_delete_event(|_, _| {
      gtk::main_quit();
      Inhibit(true)
    });
    gtk::main();
  }
}