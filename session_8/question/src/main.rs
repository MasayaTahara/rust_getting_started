fn main() {
    // Q. commandとvalueがセットになったorderがある。
    // command "insert" は、続くvalueを配列の末尾に追加し、
    // command "delete" は、配列の中から続くvalueに合致するものがあればその先頭を削除する。
    // 例えば、orderが
    // insert 1 insert 2 insert 1 delete 1
    // のとき、配列は最終的に 2 1 となる。
    // 以下のorderが与えられるとき、配列が最終的にどうなるか求めなさい。
    let order = vec![
        ("insert", 3),
        ("insert", 2),
        ("insert", 3),
        ("insert", 1),
        ("delete", 3),
        ("insert", 2),
        ("delete", 2),
    ];
    // 制約1: Orderという構造体を作り、commandとvalueというフィールドを持たせること。
    // 制約2: 上記変数orderをcommandとvalueのセット（Orderインスタンス）に分割して、それをVecとしてまとめること。
    // 制約3: execute_orderという関数を作成し、引数として上記Vecを渡して、最終結果を求めること。
    // 注意: 問題文中では配列と言っているが、Rustの場合は可変長配列はVecなので、最終結果はVecとして出力する形で良い。
    // ヒント: 上記のorderは以下のように分割可能。
    for o in order {
        println!("command is {}, value is {}", o.0, o.1);
    }
}
