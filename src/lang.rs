//! The messages of the mod, in English, Russian and Ukrainian. The server sends each player the text in the
//! language of their game (English for other languages), so players do not need the mod on their client.
//!
//! A text can have places for values: `{owner}` takes the value named `owner`.

/// A language of the messages.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Lang {
    #[default]
    En,
    Ru,
    Uk,
}

impl Lang {
    /// The language of a game language code (`uk_ua`, `ru_ru`, `en_us`...).
    pub fn of(code: &str) -> Lang {
        let code = code.to_ascii_lowercase();
        if code.starts_with("uk") {
            Lang::Uk
        } else if code.starts_with("ru") {
            Lang::Ru
        } else {
            Lang::En
        }
    }
}

macro_rules! messages {
    ($($key:ident: $en:literal, $ru:literal, $uk:literal;)*) => {
        /// A message of the mod.
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum Key { $($key,)* }

        impl Key {
            pub const ALL: &[Key] = &[$(Key::$key,)*];

            /// The text of the message in `lang`, with its places.
            pub fn text(self, lang: Lang) -> &'static str {
                match (self, lang) {
                    $(
                        (Key::$key, Lang::En) => $en,
                        (Key::$key, Lang::Ru) => $ru,
                        (Key::$key, Lang::Uk) => $uk,
                    )*
                }
            }
        }
    };
}

messages! {
    NotTrusted:
        "This land is claimed by {owner}. They have not trusted you here.",
        "Эта земля — приват игрока {owner}. Он не дал вам здесь доступа.",
        "Ця земля — приват гравця {owner}. Він не дав вам тут доступу.";
    NotYourBody:
        "This body is not yours. This land is claimed by {owner}.",
        "Это не ваше тело. Эта земля — приват игрока {owner}.",
        "Це не ваше тіло. Ця земля — приват гравця {owner}.";
    OwnedBy:
        "This land is claimed by {owner}.",
        "Эта земля — приват игрока {owner}.",
        "Ця земля — приват гравця {owner}.";
    NotClaimed:
        "This land is not claimed.",
        "Эта земля никому не принадлежит.",
        "Ця земля нікому не належить.";
    Overlaps:
        "This claim overlaps the claim of {owner}.",
        "Этот приват заходит на приват игрока {owner}.",
        "Цей приват заходить на приват гравця {owner}.";
    TooSmall:
        "A claim must be at least {width} blocks wide and {area} blocks in area.",
        "Приват должен быть не уже {width} блоков и не меньше {area} блоков по площади.",
        "Приват має бути не вужчим за {width} блоків і не меншим за {area} блоків за площею.";
    NotEnoughBlocks:
        "This claim needs {needed} claim blocks. You have {left}.",
        "Этому привату нужно {needed} блоков привата. У вас есть {left}.",
        "Цьому привату потрібно {needed} блоків привату. У вас є {left}.";
    Claimed:
        "Claim made: {area} blocks. Claim blocks left: {left}.",
        "Приват создан: {area} блоков. Осталось блоков привата: {left}.",
        "Приват створено: {area} блоків. Залишилось блоків привату: {left}.";
    Resized:
        "Claim changed: {area} blocks. Claim blocks left: {left}.",
        "Приват изменён: {area} блоков. Осталось блоков привата: {left}.",
        "Приват змінено: {area} блоків. Залишилось блоків привату: {left}.";
    FirstCorner:
        "First corner marked. Now right-click the opposite corner with the golden shovel.",
        "Первый угол отмечен. Теперь нажмите золотой лопатой на противоположный угол.",
        "Перший кут позначено. Тепер натисніть золотою лопатою на протилежний кут.";
    ResizeStart:
        "Right-click the new place of this corner with the golden shovel.",
        "Нажмите золотой лопатой на новое место этого угла.",
        "Натисніть золотою лопатою на нове місце цього кута.";
    NotInClaim:
        "You are not in a claim.",
        "Вы не в привате.",
        "Ви не в приваті.";
    OnlyOwnerRemoves:
        "Only the owner of this claim can remove it.",
        "Удалить этот приват может только его владелец.",
        "Видалити цей приват може лише його власник.";
    Abandoned:
        "Claim removed. Claim blocks left: {left}.",
        "Приват удалён. Осталось блоков привата: {left}.",
        "Приват видалено. Залишилось блоків привату: {left}.";
    ConfirmAbandonAll:
        "This removes all your claims ({count}). Run the command again within 30 seconds to confirm.",
        "Это удалит все ваши приваты ({count}). Повторите команду в течение 30 секунд, чтобы подтвердить.",
        "Це видалить усі ваші привати ({count}). Повторіть команду протягом 30 секунд, щоб підтвердити.";
    AbandonedAll:
        "All your claims are removed ({count}). Claim blocks left: {left}.",
        "Все ваши приваты удалены ({count}). Осталось блоков привата: {left}.",
        "Усі ваші привати видалено ({count}). Залишилось блоків привату: {left}.";
    NoClaims:
        "You have no claims.",
        "У вас нет приватов.",
        "У вас немає приватів.";
    UnknownPlayer:
        "No player with the name {name} has been on this server.",
        "Игрок с ником {name} ещё не заходил на этот сервер.",
        "Гравець з ніком {name} ще не заходив на цей сервер.";
    NeedManager:
        "You need manager trust in this claim to do this.",
        "Для этого нужен доступ управляющего в этом привате.",
        "Для цього потрібен доступ керівника в цьому приваті.";
    OnlyOwnerManager:
        "Only the owner of the claim can give manager trust.",
        "Доступ управляющего может дать только владелец привата.",
        "Доступ керівника може дати лише власник привату.";
    Trusted:
        "{name}: {level}. Claims changed: {count}.",
        "{name}: {level}. Изменено приватов: {count}.",
        "{name}: {level}. Змінено приватів: {count}.";
    Untrusted:
        "{name}: no trust. Claims changed: {count}.",
        "{name}: доступ убран. Изменено приватов: {count}.",
        "{name}: доступ прибрано. Змінено приватів: {count}.";
    TrustAccess:
        "access (doors, buttons, beds)",
        "доступ к дверям, кнопкам и кроватям",
        "доступ до дверей, кнопок і ліжок";
    TrustContainer:
        "containers and animals",
        "доступ к сундукам и животным",
        "доступ до скринь і тварин";
    TrustBuild:
        "building",
        "право строить",
        "право будувати";
    TrustManager:
        "manager (can trust others)",
        "управляющий (может давать доступ)",
        "керівник (може давати доступ)";
    TrustListHead:
        "Claim of {owner}:",
        "Приват игрока {owner}:",
        "Приват гравця {owner}:";
    TrustListLine:
        "{level}: {names}",
        "{level}: {names}",
        "{level}: {names}";
    TrustListEmpty:
        "Nobody is trusted here.",
        "Здесь ни у кого нет доступа.",
        "Тут ні в кого немає доступу.";
    BlocksOnline:
        "Claim blocks of {name}: {earned} (bonus {bonus}), used {used}, left {left}.",
        "Блоки привата {name}: {earned} (бонус {bonus}), занято {used}, осталось {left}.",
        "Блоки привату {name}: {earned} (бонус {bonus}), зайнято {used}, залишилось {left}.";
    BlocksOffline:
        "Claim blocks of {name}: bonus {bonus}, used {used}. The player is not online.",
        "Блоки привата {name}: бонус {bonus}, занято {used}. Игрок не в сети.",
        "Блоки привату {name}: бонус {bonus}, зайнято {used}. Гравець не в мережі.";
    ClaimLine:
        "{dimension} {x1} {z1} to {x2} {z2}: {area} blocks",
        "{dimension} от {x1} {z1} до {x2} {z2}: {area} блоков",
        "{dimension} від {x1} {z1} до {x2} {z2}: {area} блоків";
    BonusSet:
        "{name} now has {bonus} bonus claim blocks.",
        "У {name} теперь {bonus} бонусных блоков привата.",
        "У {name} тепер {bonus} бонусних блоків привату.";
    PlayersOnly:
        "Only a player can run this command.",
        "Эту команду может выполнить только игрок.",
        "Цю команду може виконати лише гравець.";
    ConfirmButton:
        "[confirm]",
        "[подтвердить]",
        "[підтвердити]";
    ConfirmHover:
        "Click to remove all your claims",
        "Нажмите, чтобы удалить все ваши приваты",
        "Натисніть, щоб видалити всі ваші привати";
}

/// What a message is: it gives the colours of the message in chat, as GriefDefender has them.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Kind {
    /// Something is refused: red, the values gold.
    Refusal,
    /// Something is done: green, the values yellow.
    Done,
    /// Information: aqua, the values white.
    Info,
}

impl Kind {
    /// The colour of the text, and of the values in it.
    pub fn colors(self) -> (&'static str, &'static str) {
        match self {
            Kind::Refusal => ("red", "gold"),
            Kind::Done => ("green", "yellow"),
            Kind::Info => ("aqua", "white"),
        }
    }
}

impl Key {
    pub fn kind(self) -> Kind {
        use Key::*;
        match self {
            NotTrusted | NotYourBody | Overlaps | TooSmall | NotEnoughBlocks | NotInClaim | OnlyOwnerRemoves
            | NoClaims | UnknownPlayer | NeedManager | OnlyOwnerManager | PlayersOnly | ConfirmAbandonAll => {
                Kind::Refusal
            }
            Claimed | Resized | Abandoned | AbandonedAll | Trusted | Untrusted | BonusSet => Kind::Done,
            _ => Kind::Info,
        }
    }
}

/// A part of a message: plain text, or a value put in a place.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Piece {
    pub text: String,
    pub value: bool,
}

/// The text of `key` in `lang` in parts: the text between the places, and the value of each place.
pub fn pieces(lang: Lang, key: Key, values: &[(&str, &dyn std::fmt::Display)]) -> Vec<Piece> {
    let mut parts = Vec::new();
    let mut rest = key.text(lang);
    while let Some(start) = rest.find('{') {
        let Some(end) = rest[start..].find('}').map(|e| start + e) else {
            break;
        };
        let name = &rest[start + 1..end];
        match values.iter().find(|(n, _)| *n == name) {
            Some((_, value)) => {
                if start > 0 {
                    parts.push(Piece {
                        text: rest[..start].to_owned(),
                        value: false,
                    });
                }
                parts.push(Piece {
                    text: value.to_string(),
                    value: true,
                });
            }
            None => parts.push(Piece {
                text: rest[..=end].to_owned(),
                value: false,
            }),
        }
        rest = &rest[end + 1..];
    }
    if !rest.is_empty() {
        parts.push(Piece {
            text: rest.to_owned(),
            value: false,
        });
    }
    parts
}

fn colored(text: impl Into<String>, color: &str) -> mod_api::Text {
    let mut style = mod_api::text::TextStyle::default();
    style.color = Some(color.to_owned());
    mod_api::Text::literal(text).with_style(style)
}

/// The message `key` in `lang` for chat or the action bar: coloured by its kind, the values in their own colour.
pub fn styled(lang: Lang, key: Key, values: &[(&str, &dyn std::fmt::Display)]) -> mod_api::Text {
    let (text, value) = key.kind().colors();
    pieces(lang, key, values)
        .into_iter()
        .fold(colored("", text), |line, piece| {
            line.append(colored(piece.text, if piece.value { value } else { text }))
        })
}

/// A button in chat: `label` in green, `hover` shown on it, and `command` run on a click.
pub fn button(lang: Lang, label: Key, hover: Key, command: &str) -> mod_api::Text {
    let mut style = mod_api::text::TextStyle::default();
    style.color = Some("green".to_owned());
    style.bold = Some(true);
    style.click = Some(mod_api::text::TextClick::RunCommand(command.to_owned()));
    style.hover = Some(mod_api::TextHover::Text(Box::new(colored(hover.text(lang), "gray"))));
    mod_api::Text::literal(label.text(lang)).with_style(style)
}

/// The text of `key` in `lang`, with each `{name}` place set from `values`.
pub fn say(lang: Lang, key: Key, values: &[(&str, &dyn std::fmt::Display)]) -> String {
    let mut text = key.text(lang).to_owned();
    for (name, value) in values {
        text = text.replace(&format!("{{{name}}}"), &value.to_string());
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_message_has_three_texts_with_the_same_places() {
        let places = |text: &str| {
            let mut found: Vec<String> = text
                .split('{')
                .skip(1)
                .filter_map(|part| part.split_once('}').map(|(name, _)| name.to_owned()))
                .collect();
            found.sort();
            found
        };
        for key in Key::ALL {
            let en = places(key.text(Lang::En));
            for lang in [Lang::Ru, Lang::Uk] {
                assert!(!key.text(lang).is_empty(), "{key:?} {lang:?}");
                assert_eq!(places(key.text(lang)), en, "{key:?} {lang:?}");
            }
        }
    }

    #[test]
    fn a_message_is_split_into_text_and_values() {
        let parts = pieces(Lang::En, Key::NotEnoughBlocks, &[("needed", &100), ("left", &40)]);
        let text: Vec<(&str, bool)> = parts.iter().map(|p| (p.text.as_str(), p.value)).collect();
        assert_eq!(
            text,
            vec![
                ("This claim needs ", false),
                ("100", true),
                (" claim blocks. You have ", false),
                ("40", true),
                (".", false)
            ]
        );
        let plain: String = pieces(Lang::Ru, Key::OwnedBy, &[("owner", &"Alex")])
            .into_iter()
            .map(|p| p.text)
            .collect();
        assert_eq!(plain, say(Lang::Ru, Key::OwnedBy, &[("owner", &"Alex")]));
        assert_eq!(Key::NotTrusted.kind(), Kind::Refusal);
        assert_eq!(Key::Claimed.kind(), Kind::Done);
        assert_eq!(Key::FirstCorner.kind(), Kind::Info);
    }

    #[test]
    fn the_language_comes_from_the_game_language_code() {
        assert_eq!(Lang::of("uk_ua"), Lang::Uk);
        assert_eq!(Lang::of("ru_RU"), Lang::Ru);
        assert_eq!(Lang::of("de_de"), Lang::En);
        assert_eq!(
            say(Lang::Uk, Key::OwnedBy, &[("owner", &"Alex")]),
            "Ця земля — приват гравця Alex."
        );
    }
}
