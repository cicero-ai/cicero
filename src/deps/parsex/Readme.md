
# Simplistic, Efficient HTML Parser and Modifier

Provides a simplistic, quick and efficient way to parse HTML documents plus optionally modify and rebuild them with modifications if desired.

## Examples

```
    // Parse HTML code
    let html = "<h1>Testing parsex</h1><p class=\"headline\">Quick test of this Rust package</p><br /><br /><ul id=\"category-list\"><li>First Item</li><li>Second Item</li><li>Third Item</li></ul><br /><p>Feel free to <a href-\"/contact\" id=\"contact-link\">contact us</a>.</p>";
    let mut stack = parsex::parse_html(html);

    // Go through all tags in hierarchial order, top to bottom, left to right
    for tag in stack.iter() {
        println!("Tag: {}, Contents: {}", tag.tag(), tag.contents());

        // Update, if title
        if tag.tag() == "h1" {
            tag.as_mut(&mut stack).set_contents("Updated Title Here");
        }
    }

    // Get updated title contents
    let title = stack.query().tag("h1").to_vec();
    println!("New Title: {}", title[0].contents());

    // Go through all lists, update 'class' attribute on items to 'new-item'
    for ul in stack.query().tag("ul").iter() {
        println!("List ID: {}", ul.attr("id").unwrap());
        for item in ul.children(&mut stack).iter() {
            println!("Item: {}", item.contents());
            //item.as_mut(&mut stack).set_attr("class", "new-item");
        }
    }

    // Get contact link, update href attribute
    if let Some(contact) = stack.query().tag("a").id("contact-link").iter().next() {
        let tag = stack.get_mut(&contact.id()).unwrap();
        tag.set_attr("href", "/new-contact");
    }

    // Render HTML document with modifications
    let updated_html = stack.render();
    println!("Updated HTML\n\n{}\n", updated_html);
```


## Contact

If you need any assistance or software development done, contact me via e-mail at matt@apexpl.io.

