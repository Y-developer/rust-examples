fn main() {
    // elem යන්න u8 එකක් බව compiler එක හොදටම දනී.
    let elem = 5u8;

    // empty vector එකක් සෑදීම. මෙය growable array එකකි.
    // සාමාන්‍ය මෙහිදී මෙම array එකේ අඩංගු වන element වල type සදහන් කල යුතුය.
    // ඒ සදහා Vec::<T> යන්න භාවිතා කල හැක. T යනු type එකයි.
    let mut vec = Vec::new();

    // දැන් elem vec ට inset කරමු.
    vec.push(elem);
    // දැන් vec u8 type එකේ බව compiler එක දැනගනී. මේ නිසා compile කිරීමේදී error එකක් ඇති නොවේ.
    // සිදුවන්නේ කුමක් දැයි ^ vec.push(elem); comment කර බලන්න.

    println!("vec = {:?}", vec);

    // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
    // vec_1 යනු type එකක් සමග සාදන ලද vector එකකි.
    let mut vec_1 = Vec::<u8>::new();
    vec_1.push(elem);
    println!("vec_1 = {:?}", vec_1);
}