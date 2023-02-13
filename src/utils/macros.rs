macro_rules! map {
    ($($k:expr => $v:expr),* $(,)?) => {{
		let mut m = ::std::collections::BTreeMap::new();
        $(m.insert($k.into(), $v.into());)+
        m
    }};
  }
pub(crate) use map;