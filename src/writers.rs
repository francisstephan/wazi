use crate::dbase;
use web_sys::window;

pub fn printcandidatelist(chain: &str) {
    let answer = dbase::getcandidatelist(chain);
    let mut resp: String;
    if answer.is_empty() {
        resp = "<br /><br />No hanzi available for request".to_owned()
    } else {
        resp = String::from("<br />Select one hanzi from this list:<br>");
        resp.push_str(&answer)
    }
    let cont = window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("zilist")
        .unwrap();
    cont.set_inner_html(&resp);
}

pub fn printzidata(zistring: String) {
    let mut resp: String = "".to_owned();
    if zistring.len() == 0 {
        resp.push_str("<p>No hanzi in Result</p>");
    } else {
        let carac = zistring.chars().last().unwrap();
        resp.push_str(format!("<p>Data for character {} :</p>", carac).as_str());
        let vec = dbase::zilist(carac);

        resp.push_str(
            "<table><tr><td>Strokes</td><td>Pinyin</td><td>Unicode</td><td>Translation</td></tr>",
        );
        for zi in vec {
            resp.push_str(&format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                zi.strokes, zi.pinyin_ton, zi.unicode, zi.sens
            ));
        }
        resp.push_str("</table>")
    }
    let cont = window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("zilist")
        .unwrap();
    cont.set_inner_html(&resp);
}
