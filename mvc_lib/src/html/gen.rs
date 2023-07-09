use rand::prelude::*;


pub struct HtmlGenerator {

}

impl HtmlGenerator {
    pub fn new() -> HtmlGenerator {
        HtmlGenerator {

        }
    }

    pub fn generate(&self) -> String {
        let mut html = String::new();
        let mut rng = rand::thread_rng();

        let is_page = rng.gen::<f32>() > 0.5;

        if is_page {
            html.push_str("<!DOCTYPE html>");
            html.push_str("<html>");
            html.push_str("<body>");
        }

        html.push_str("
        <div>
            <ul>
                <li>test</li>
            </ul>
            <p>test</p>
            <br/>
            <input type=\"text\" />
            <table>
                <tr>
                    <td>test</td>
                </tr>
            </table>
            <hr>
            <img src=\"test.png\" />
            <a href=\"test.html\">test</a>
            <form action=\"test.html\" method=\"post\">
                <input type=\"text\" />
                <input type=\"submit\" />
            </form>
        </div>"
        );
        
        if is_page {
            html.push_str("</body>");
            html.push_str("</html>");
        }

        html
    }
}