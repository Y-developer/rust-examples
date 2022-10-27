mod my {
    // public structure එකක් තුල ඇති public field එකක් (generic type `T`)
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // public structure එකක් තුල ඇති private field එකක් (generic type `T`)
    #[allow(dead_code)]
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        // ClosedBox වලට ඇති public constructor method එක
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents: contents,
            }
        }
    }
}

fn main() {
    // OpenBox structure එක public හා එය තුල ඇති field එක public නිසා
    // විශේෂිත costructor method එකක් නොමැතිව object එකක් සෑදීමට හැකිය.
    let open_box = my::OpenBox { contents: "public information" };

    // එසේම OpenBox වල field එක access කිරීමටද හැකිය.
    println!("The open box contains: {}", open_box.contents);

    // ClosedBox structure එක public වුවත් එහි ඇති field එක private නිසා සාමාන්‍ය පරිදි object එකක් තැනීමට නොහැකිය.
    // Error! `ClosedBox` has private fields
    //let closed_box = my::ClosedBox { contents: "classified information" };
    // TODO ^ Try uncommenting this line

    // නමුත් ClosedBox වල constructor method public නිසා එය භාවිතා කර object එකක් සෑදීමට හැකිය.
    let _closed_box = my::ClosedBox::new("classified information");

    // නමුත් සාදාගත් ClosedBox object එකේ field එක කියවීමට නොහැක.
    // Error! The `contents` field is private
    //println!("The closed box contains: {}", _closed_box.contents);
    // TODO ^ Try uncommenting this line
}