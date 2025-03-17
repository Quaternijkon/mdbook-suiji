// lib.rs

use mdbook::{book::{Book, Chapter}, errors::Error, preprocess::{Preprocessor, PreprocessorContext}, BookItem};
use rand::Rng; // 引入随机数生成库

#[derive(Default)]
pub struct RandomNumberPreprocessor;

impl RandomNumberPreprocessor {
    pub fn new() -> RandomNumberPreprocessor {
        RandomNumberPreprocessor
    }
}

impl Preprocessor for RandomNumberPreprocessor {
    fn name(&self) -> &str {
        "suiji"
    }

    fn run(&self, ctx: &PreprocessorContext, book: Book) -> Result<Book, Error> {
        let mut book = book;

        book.for_each_mut(|item: &mut BookItem| {
            if let BookItem::Chapter(ref mut chapter) = *item {
                if let Err(err) = handle_chapter(chapter) {
                    eprintln!("Error processing chapter: {}", err);
                }
            }
        });

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}

fn handle_chapter(chapter: &mut Chapter) -> Result<(), Error> {
    // 生成一个随机数
    let random_number: i32 = rand::thread_rng().gen_range(i32::MIN..=i32::MAX);

    // 将内容中的 {{ #random_number }} 替换为生成的随机数
    chapter.content = chapter
        .content
        .replace("{{ #random_number }}", &random_number.to_string());

    Ok(())
}
