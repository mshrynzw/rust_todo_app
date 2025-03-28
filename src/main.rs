// ユーザーからの入力を扱うための標準ライブラリを使う
use std::io;
// VecDeque（両端キュー）を使うためのライブラリ
use std::collections::VecDeque;

// タスクを表す構造体
#[derive(Debug)] // デバッグ出力をできるようにする
struct Task {
    title: String, // タスクのタイトル
    done: bool,    // 完了したかどうか
}

// Taskのメソッドを定義する
impl Task {
    // 新しいタスクを作るための関数（タイトルを渡す）
    fn new(title: String) -> Self {
        Self {
            title,
            done: false, // 最初は未完了
        }
    }

    // タスクを完了済みにする関数
    fn mark_done(&mut self) {
        self.done = true;
    }
}

fn main() {
    // タスクリストを作成（両端キューで柔軟に追加・削除できる）
    let mut tasks: VecDeque<Task> = VecDeque::new();

    // 無限ループ（終了するまで続ける）
    loop {
        // メニューの表示
        println!("\n--- ToDoアプリ ---");
        println!("1. タスクを追加");
        println!("2. タスク一覧を表示");
        println!("3. タスクを完了にする");
        println!("4. 終了");

        // ユーザーの入力を受け取る（文字列）
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap(); // 入力を読み取り
        let input = input.trim(); // 改行を取り除く

        // 入力に応じて処理を分岐する
        match input {
            "1" => {
                // タスクのタイトルを入力してもらう
                println!("タスクのタイトルを入力してください:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                let title = title.trim().to_string(); // 改行除去してStringに変換

                // 新しいタスクを作ってリストに追加
                let task = Task::new(title);
                tasks.push_back(task);

                println!("追加しました！");
            }
            "2" => {
                // タスクリストを表示
                for (i, task) in tasks.iter().enumerate() {
                    let status = if task.done { "[完了]" } else { "[未完了]" };
                    println!("{}: {} {}", i + 1, status, task.title);
                }
            }
            "3" => {
                // 完了にしたいタスクの番号を入力してもらう
                println!("完了にしたいタスクの番号を入力してください:");
                let mut num = String::new();
                io::stdin().read_line(&mut num).unwrap();

                // 入力を数値に変換（失敗するとエラーになる）
                if let Ok(index) = num.trim().parse::<usize>() {
                    // 入力された番号に該当するタスクを取得
                    if let Some(task) = tasks.get_mut(index - 1) {
                        task.mark_done();
                        println!("完了にしました！");
                    } else {
                        println!("番号が不正です。");
                    }
                } else {
                    println!("数字を入力してください。");
                }
            }
            "4" => {
                // 終了処理
                println!("終了します。");
                break; // ループを抜けて終了
            }
            _ => {
                // その他の入力（無効）
                println!("無効な入力です。");
            }
        }
    }
}
