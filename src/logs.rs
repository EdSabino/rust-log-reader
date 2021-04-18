use std::io::Read;
use std::fs::File;
use std::collections::HashMap;
use crate::transaction::Transaction;

pub struct Logs {
    filename: String,
    content: String,
    pub table_line: String,
    pub commited: Vec<String>
}

impl Logs {

    pub fn new(filename: String) -> Self {
        Logs {
            filename: filename,
            content: "".to_string(),
            table_line: "".to_string(),
            commited: Vec::new()
        }
    }

    pub fn load(&mut self) {
        let mut file = File::open(self.filename.as_str()).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        self.table_line = contents.split("\n").collect::<Vec<_>>()[0].to_string();
        self.content = contents;
    }

    pub fn get_value(&self, key: &String) -> String {
        let mut finded_value = "".to_string();
        for value in self.table_line.split(" | ").collect::<Vec<_>>() {
            let splited = value.split("=").collect::<Vec<_>>();
            let act_key = splited[0].to_string().to_lowercase();
            if act_key.as_str() == key.as_str() {
                finded_value = splited[1].to_string();
                break;
            }
        }
        finded_value
    }

    pub fn get_updatables(&self, key: &String) -> Vec<i32> {
        let mut vec = Vec::new();
        for value in self.table_line.split(" | ").collect::<Vec<_>>() {
            let splited = value.split("=").collect::<Vec<_>>();
            let act_key = splited[0].to_string().to_lowercase();
            if act_key.as_str() != key.as_str() {
                vec.push(splited[1].parse().unwrap());
            }
        }
        vec
    }

    pub fn parse_logs(&mut self) -> HashMap<String, Transaction> {
        let lines = self.content.split("\n").collect::<Vec<_>>();
        let mut transactions: HashMap<String, Transaction> = HashMap::new();
        for i in 1..lines.len()-1 {
            match &lines[i][1..7] {
                "start " => {
                    let len = lines[i].len();
                    let tr = Transaction::new();
                    transactions.insert(lines[i][7..len-2].to_string(), tr);
                },
                "Start " => {
                    for (_, tr) in transactions.iter_mut() {
                        if tr.commited {
                            tr.finalize();
                        }
                    }
                },
                "commit" => {
                    let len = lines[i].len();
                    transactions.get_mut(&lines[i][8..len-2]).unwrap().mark_commited(); 
                    self.commited.push(lines[i][8..len-2].to_string());
                },
                "End CK" => {},
                _ => {
                    let len = lines[i].len();
                    let useful_line = &lines[i][1..len-2].split(",").collect::<Vec<_>>();
                    transactions.get_mut(useful_line[0]).unwrap().change(useful_line[1].to_string(), useful_line[2].parse().unwrap());
                }
            }
        }
        transactions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_log_test() -> Logs {
        Logs::new("teste.log".to_string())
    }

    #[test]
    fn test_file_content() {
        let mut logs = create_log_test();
        assert_eq!(logs.content, "");
        logs.load();
        assert_eq!(logs.content, "ok!");
    }

    #[test]
    fn test_get_value() {
        let logs = Logs {
            filename: "teste.log".to_string(),
            content: "".to_string(),
            table_line: "A=20 | B=20 | C=70 | D=50 | E=17 | F=1".to_string(),
            commited: Vec::new()
        };

        assert_eq!(logs.get_value(&"a".to_string()), 20.to_string());
        assert_eq!(logs.get_value(&"b".to_string()), 20.to_string());
        assert_eq!(logs.get_value(&"c".to_string()), 70.to_string());
        assert_eq!(logs.get_value(&"d".to_string()), 50.to_string());
        assert_eq!(logs.get_value(&"e".to_string()), 17.to_string());
        assert_eq!(logs.get_value(&"f".to_string()), 1.to_string());
    }
}