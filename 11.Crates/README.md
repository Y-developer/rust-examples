# 11. Crates

rust වල compile කිරීමට බලාපොරොත්තු වන unit එකක් ලෙස හදුන්වයි. ඕනෑම අවස්ථාවක `rustc some_file.rs` commond එක භාවිතා කරන්නේ නම් මෙහි `some_file.rs` යන්න crate එකක් වේ.
<br>
<br>
crate එකක් තුල වෙනත් modules (mod keyword එක) භාවිතා කර ඇත්නම් ඒවා වෙනුවට එම modules වල අන්තර්ගතය ආදේශ වී සියල්ල එක පද්දතියක් ලෙස compile වීම සිදු කරයි. *(nodejs webpack වල bundle සාදන ආකාරයට මෙන්)* ඒ අනුව crate එකක් යනු module කීපයක් එකතු කරගත් තනි පද්දතියක් ලෙස සැලකිය හැක.

`workspace`  --->  `packages`  --->  `crates`  --->  `modules`

<br>
create එකක් binary format එකට හෝ library එක්ක ලෙස compile කල හැක.
<br><br>

By default crate එකක් binary format එකට compile වේ. 
```
rustc some_file.rs
```

crate එක library එකක් බවට compile කිරීමට `--crate-type` flag එකට `lib` ලබා දිය යුතුය.
```
rustc some_file.rs --crate-type=lib
```

package එකක් තුල අන්තර්ගත කල හැක්කේ එක් library crate එකක් පමණි. නමුත් binary crate කිහිපයක් package එක්ක තුල අන්තර්ගත කල හැක.


