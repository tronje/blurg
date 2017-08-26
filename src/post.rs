use std::fs::{self, File};
use std::io::{BufRead, BufReader};


#[derive(Serialize)]
pub struct Post {
    pub meta: PostMeta,
    pub content: String,
}

impl Post {
    pub fn from_id(id: usize) -> Option<Post> {
        let meta = match PostMeta::from_id(id) {
            Some(m) => m,
            None => return None,
        };

        let content = match Post::parse_content(id) {
            Some(c) => c,
            None => return None,
        };

        Some(Post {
            meta: meta,
            content: content,
        })
    }

    fn parse_content(id: usize) -> Option<String> {
        let mut result = String::new();

        let f = match File::open(format!("www/posts/{}", id)) {
            Ok(file) => file,
            Err(_) => return None,
        };

        let mut file = BufReader::new(&f);
        let mut lines = file.lines();
        
        lines.next(); // first line
        lines.next(); // second line
        lines.next(); // third line

        for line in lines {
            result.push_str(&line.unwrap());
        }

        Some(result)
    }

    pub fn list_posts() -> Vec<usize> {
        let dir = fs::read_dir("www/posts/").unwrap();
        let mut posts = Vec::new();

        for entry in dir {
            posts.push(entry.unwrap());
        }

        let mut result = Vec::new();

        for post in posts {
            result.push(post.file_name().into_string().unwrap().parse::<usize>().unwrap());
        }

        result
    }
}


#[derive(Serialize)]
pub struct PostMeta {
    pub id: usize,
    pub title: String,
    pub author: String,
}

impl PostMeta {
    pub fn from_id(post_id: usize) -> Option<PostMeta> {
        let f = match File::open(format!("www/posts/{}", post_id)) {
            Ok(file) => file,
            Err(_) => return None,
        };

        let mut file = BufReader::new(&f);
        let mut lines = file.lines();

        let title = lines.next().unwrap().unwrap();
        let author = lines.next().unwrap().unwrap();

        Some(PostMeta {
            id: post_id,
            title: title,
            author: author,
        })
    }
}
