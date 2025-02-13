use jieba_rs::Jieba;
use phper::{
    arrays::{InsertKey, ZArray},
    classes::{ClassEntity, Visibility},
    functions::Argument,
};

pub const JIEBA_CLASS_NAME: &str = "Jieba";

pub fn make_jieba_class() -> ClassEntity<Jieba> {
    let mut class = ClassEntity::<Jieba>::new_with_default_state_constructor(JIEBA_CLASS_NAME);

    class
        .add_method("cut", Visibility::Public, |this, arguments| {
            let sentence = arguments[0].expect_z_str()?.to_str()?;
            let hmm = arguments
                .get(1)
                .map(|b| b.expect_bool())
                .transpose()?
                .unwrap_or_default();

            let words = this.as_state().cut(sentence, hmm);

            let mut result = ZArray::with_capacity(words.len());
            for word in words {
                result.insert(InsertKey::NextIndex, word);
            }

            phper::ok(result)
        })
        .argument(Argument::by_val("sentence"))
        .argument(Argument::by_val_optional("hmm"));

    class
        .add_method("cutAll", Visibility::Public, |this, arguments| {
            let sentence = arguments[0].expect_z_str()?.to_str()?;

            let words = this.as_state().cut_all(sentence);

            let mut result = ZArray::with_capacity(words.len());
            for word in words {
                result.insert(InsertKey::NextIndex, word);
            }

            phper::ok(result)
        })
        .argument(Argument::by_val("sentence"));

    class
        .add_method("cutForSearch", Visibility::Public, |this, arguments| {
            let sentence = arguments[0].expect_z_str()?.to_str()?;
            let hmm = arguments
                .get(1)
                .map(|b| b.expect_bool())
                .transpose()?
                .unwrap_or_default();

            let words = this.as_state().cut_for_search(sentence, hmm);

            let mut result = ZArray::with_capacity(words.len());
            for word in words {
                result.insert(InsertKey::NextIndex, word);
            }

            phper::ok(result)
        })
        .argument(Argument::by_val("sentence"))
        .argument(Argument::by_val_optional("hmm"));

    class
}
