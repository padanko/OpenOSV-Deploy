use regex;
use crate::defines;

// parseのやつとは違っていつレンダリングしても不変なテキストを扱う
// 備考:　これらの関数は1回のアクセスごとに1回しか呼ばれない

fn reply_com_replace(mut text: String) -> String {
    let reply_regex = regex::Regex::new(r"&gt;&gt;([0-9]+)").unwrap(); // 従来のOpen2chと同じように >>1 などで返信できるようにする
    text = reply_regex.replace_all(&text.as_str(), |caps: &regex::Captures| {
        format!("<a href='#r{}'>&gt;&gt;{}</a>", &caps[1], &caps[1])
    }).to_string();

    text

}

fn img_com_replace(mut text: String) -> String {
    let img_regex = regex::Regex::new("!Img:\"([^\"]+)\"").unwrap(); // ...
    
    text = img_regex.replace_all(&text.as_str(), |caps: &regex::Captures| {
        format!("<img src='#r{}'>", &caps[1])
    }).to_string();

    text

}

pub fn replace_text(mut text: String) -> String {

    text = reply_com_replace(text);
    text = img_com_replace(text);

    text = text.replace("!version", format!("!version\n<i>OpenOSV {}</i>", defines::VERSION).as_str());
    text = text.replace("\n", "<br>");


    
    text
}