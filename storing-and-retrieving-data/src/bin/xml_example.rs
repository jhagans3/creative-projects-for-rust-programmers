use xml::reader::{EventReader, XmlEvent};

#[derive(Debug, Default)]
struct Product {
    id: u32,
    category: String,
    name: String,
}

#[derive(Debug, Default)]
struct Sale {
    id: String,
    product_id: u32,
    date: i64,
    quantity: f64,
    unit: String,
}

// indicates the current position of
// the parsing in general. We can be
// inside a product (InProduct),
// inside a sale (InSale),
// or outside of both (Other)
enum LocationItem {
    Other,
    InProduct,
    InSale,
}

// If we are inside a product,
// the LocationProduct enum indicates
// the current position of parsing inside
// the current product. This can be within
// any of the allowed fields or outside of
// all of them. Similar states happen for sales.
enum LocationProduct {
    Other,
    InId,
    InCategory,
    InName,
}
enum LocationSale {
    Other,
    InId,
    InProductId,
    InDate,
    InQuantity,
    InUnit,
}

// storing-and-retrieving-data$ cargo run --bin xml_example data/sales.xml
fn main() {
    let mut location_item = LocationItem::Other;
    let mut location_product = LocationProduct::Other;
    let mut location_sale = LocationSale::Other;
    let pathname = std::env::args().nth(1).unwrap();

    // These are initialized with default values.
    // Whenever there are some characters available,
    // they are stored in the corresponding field
    // of the current struct see Ex 1.
    // situation where the value of
    // location_item is LocationItem::InProduct and the
    // value of location_product is LocationProduct::InCategory
    let mut product: Product = Default::default();
    let mut sale: Sale = Default::default();

    let file = std::fs::File::open(pathname).unwrap();
    let file = std::io::BufReader::new(file);

    // An object of the EventReader type scans the buffered file and it
    // generates an event whenever a step is performed in the parsing.
    // The application code handles these kinds of events according
    // to their needs. The word event is used by this crate, but the
    // word transition would probably be a better description
    // of the data extracted by the parser.
    let parser = EventReader::new(file);

    // XmlEvent::StartElement: Signals that an XML element is beginning.
    // It is decorated by the name of the beginning element
    // and the possible attributes of that element.
    // XmlEvent::EndElement: Signals that an XML element is ending.
    // It is decorated by the name of the ending element.
    // XmlEvent::Characters: Signals that the textual contents
    // of an element is available. It is decorated by that available text.
    for event in parser {
        match &location_item {
            LocationItem::Other => match event {
                Ok(XmlEvent::StartElement { ref name, .. }) if name.local_name == "product" => {
                    location_item = LocationItem::InProduct;
                    location_product = LocationProduct::Other;
                    product = Default::default();
                }
                Ok(XmlEvent::StartElement { ref name, .. }) if name.local_name == "sale" => {
                    location_item = LocationItem::InSale;
                    location_sale = LocationSale::Other;
                    sale = Default::default();
                }
                _ => {}
            },
            LocationItem::InProduct => match &location_product {
                LocationProduct::Other => match event {
                    Ok(XmlEvent::StartElement { ref name, .. }) if name.local_name == "id" => {
                        location_product = LocationProduct::InId;
                    }
                    Ok(XmlEvent::StartElement { ref name, .. })
                        if name.local_name == "category" =>
                    {
                        location_product = LocationProduct::InCategory;
                    }
                    Ok(XmlEvent::StartElement { ref name, .. }) if name.local_name == "name" => {
                        location_product = LocationProduct::InName;
                    }
                    Ok(XmlEvent::EndElement { .. }) => {
                        location_item = LocationItem::Other;
                        println!("  Exit product: {:?}", product);
                    }
                    _ => {}
                },
                LocationProduct::InId => match event {
                    Ok(XmlEvent::Characters(characters)) => {
                        product.id = characters.parse::<u32>().unwrap();
                        println!("Got product.id: {}.", characters);
                    }
                    Ok(XmlEvent::EndElement { .. }) => {
                        location_product = LocationProduct::Other;
                    }
                    _ => {}
                },
                LocationProduct::InCategory => match event {
                    // Ex 1. we are in a category of a product.
                    // In this situation, there can be the name
                    // of the category or the end of the category.
                    // To get the name of the category
                    Ok(XmlEvent::Characters(characters)) => {
                        // the characters variable gets the name of the
                        // category and a clone of it is assigned to the
                        // product.category field.
                        product.category = characters.clone();
                        // Then, the name is printed to the console.
                        println!("Got product.category: {}.", characters);
                    }
                    Ok(XmlEvent::EndElement { .. }) => {
                        location_product = LocationProduct::Other;
                    }
                    _ => {}
                },
                LocationProduct::InName => match event {
                    Ok(XmlEvent::Characters(characters)) => {
                        product.name = characters.clone();
                        println!("Got product.name: {}.", characters);
                    }
                    Ok(XmlEvent::EndElement { .. }) => {
                        location_product = LocationProduct::Other;
                    }
                    _ => {}
                },
            },
            LocationItem::InSale => match &location_sale {
                LocationSale::Other => match event {
                    Ok(XmlEvent::StartElement { ref name, .. }) if name.local_name == "id" => {
                        location_sale = LocationSale::InId;
                    }
                    Ok(XmlEvent::StartElement { ref name, .. })
                        if name.local_name == "product-id" =>
                    {
                        location_sale = LocationSale::InProductId;
                    }
                    Ok(XmlEvent::StartElement { ref name, .. }) if name.local_name == "date" => {
                        location_sale = LocationSale::InDate;
                    }
                    Ok(XmlEvent::StartElement { ref name, .. })
                        if name.local_name == "quantity" =>
                    {
                        location_sale = LocationSale::InQuantity;
                    }
                    Ok(XmlEvent::StartElement { ref name, .. }) if name.local_name == "unit" => {
                        location_sale = LocationSale::InUnit;
                    }
                    Ok(XmlEvent::EndElement { ref name, .. }) if name.local_name == "sale" => {
                        location_item = LocationItem::Other;
                        println!("  Exit sale: {:?}", sale);
                    }
                    _ => {}
                },
                LocationSale::InId => match event {
                    Ok(XmlEvent::Characters(characters)) => {
                        sale.id = characters.clone();
                        println!("Got sale.id: {}.", characters);
                    }
                    Ok(XmlEvent::EndElement { .. }) => {
                        location_sale = LocationSale::Other;
                    }
                    _ => {}
                },
                LocationSale::InProductId => match event {
                    Ok(XmlEvent::Characters(characters)) => {
                        sale.product_id = characters.parse::<u32>().unwrap();
                        println!("Got sale.product-id: {}.", characters);
                    }
                    Ok(XmlEvent::EndElement { .. }) => {
                        location_sale = LocationSale::Other;
                    }
                    _ => {}
                },
                LocationSale::InDate => match event {
                    Ok(XmlEvent::Characters(characters)) => {
                        sale.date = characters.parse::<i64>().unwrap();
                        println!("Got sale.date: {}.", characters);
                    }
                    Ok(XmlEvent::EndElement { .. }) => {
                        location_sale = LocationSale::Other;
                    }
                    _ => {}
                },
                LocationSale::InQuantity => match event {
                    Ok(XmlEvent::Characters(characters)) => {
                        sale.quantity = characters.parse::<f64>().unwrap();
                        println!("Got sale.quantity: {}.", characters);
                    }
                    Ok(XmlEvent::EndElement { .. }) => {
                        location_sale = LocationSale::Other;
                    }
                    _ => {}
                },
                LocationSale::InUnit => match event {
                    Ok(XmlEvent::Characters(characters)) => {
                        sale.unit = characters.clone();
                        println!("Got sale.unit: {}.", characters);
                    }
                    Ok(XmlEvent::EndElement { .. }) => {
                        location_sale = LocationSale::Other;
                    }
                    _ => {}
                },
            },
        }
    }
}
