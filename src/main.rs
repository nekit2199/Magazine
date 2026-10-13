use excelize::Spreadsheet;
use std::collections::HashMap;
use std::io;
fn main() {
    let mut temp_namecatagory = String::new();
    let path = String::from("Catalog.xlsx");
    let categories = counting_categories(path);
    println!("Введите ID категории, по которой требуется узнать остатки на складе");
    io::stdin()
        .read_line(&mut temp_namecatagory)
        .expect("Значение не введено");
    let mut namecatagory: String = temp_namecatagory.trim().to_string();
}

fn counting_balances(name_catagory: String) -> usize {
    let path = String::from("Catalog.xlsx".into());
    let wb = Spreadsheet::open_file(path);
}

fn counting_categories(path: String) -> HashMap<u8, String> {
    let mut categories_map = HashMap::new();
    let wb = Spreadsheet::open_file(path).expect("Не удалось открыть файл");
    let rows = wb
        .read("Категории товаров")
        .expect("Не удалось прочитать лист Категории товаров");

    for row in rows {
        // Safe check: убеждаемся, что в строке есть как минимум 2 заполненных колонки
        if let (Some(id_str), Some(name_str)) = (row.get(0), row.get(1)) {
            // Пробуем распарсить ID из первой колонки в число u8
            if let Ok(id) = id_str.trim().parse::<u8>() {
                // Вставляем в мапу, клонируя имя (так как оно принадлежит вектору row)
                categories_map.insert(id, name_str.clone());
            }
        }
    }

    // Возвращаем готовую HashMap (БЕЗ точки с запятой!)
    categories_map
}
