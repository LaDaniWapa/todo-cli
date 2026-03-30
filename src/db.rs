use crate::errors::TodoError;
use rusqlite::Connection;
use std::fmt;
use std::fmt::Formatter;

pub type Result<T> = std::result::Result<T, TodoError>;

pub struct Task {
    pub id: i64,
    pub title: String,
    pub done: bool,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let status = if self.done { "✓" } else { "○" };
        write!(f, "{status} [{}] {}", self.id, self.title)
    }
}

pub struct TodoDb {
    conn: Connection,
}

impl TodoDb {
    pub fn new() -> Result<TodoDb> {
        let conn = Connection::open(".todo.db")?;
        Ok(TodoDb { conn })
    }

    pub fn init(&self) -> Result<()> {
        self.conn.execute_batch("CREATE TABLE IF NOT EXISTS todos(id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT NOT NULL, done INTEGER NOT NULL DEFAULT 0)")?;
        Ok(())
    }

    pub fn add_task(&self, title: &str) -> Result<()> {
         let updated = self.conn
            .execute("INSERT INTO todos (title) values (?1)", [title])?;

        if updated > 0 {println!("Task added!")}

        Ok(())
    }
    pub fn list_tasks(&self) -> Result<Vec<Task>> {
        let mut stmt = self.conn.prepare("SELECT id, title, done FROM todos")?;

        let rows = stmt.query_map([], |row| {
            Ok(Task {
                id: row.get::<_, i64>(0)?,
                title: row.get::<_, String>(1)?,
                done: row.get::<_, i64>(2)? == 1,
            })
        })?;


        rows.collect::<rusqlite::Result<Vec<Task>>>()
            .map_err(TodoError::Db) // if there is an error, convert it to TodoErr::Db
    }

    pub fn done_task(&self, id: i64) -> Result<()> {
        let updated = self
            .conn
            .execute("UPDATE todos SET done = 1 WHERE id = (?1)", [id])?;

        if updated == 0 {
            return Err(TodoError::NotFoundTaskId(id));
        }

        Ok(())
    }

    pub fn undone_task(&self, id: i64) -> Result<()> {
        let updated = self
            .conn
            .execute("UPDATE todos SET done = 0 WHERE id = (?1)", [id])?;

        if updated == 0 {
            return Err(TodoError::NotFoundTaskId(id));
        }

        Ok(())
    }

    pub fn delete_task(&self, id: i64) -> Result<()> {
        let updated = self.conn
            .execute("DELETE FROM todos WHERE id = (?1)", [id])?;

        if updated == 0 {
            return Err(TodoError::NotFoundTaskId(id));
        }

        Ok(())
    }

    pub fn clear_done_tasks(&self) -> Result<usize> {
        let updated = self.conn.execute("DELETE FROM todos WHERE done = 1", [])?;

        Ok(updated)
    }
}
