use nom::bytes::complete::tag;
use nom::IResult;

pub fn tag_all<'a, T>(
    tags: &'a [T],
    get_tag: impl Fn(&T) -> &str,
) -> impl FnMut(&str) -> IResult<&str, &'a T>
where
{
    move |input| {
        for item in tags {
            if let Ok((rest, _)) = tag::<&str, &str, nom::error::Error<&str>>(get_tag(item))(input)
            {
                return Ok((rest, item));
            } else {
                continue;
            }
        }
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )))
    }
}
