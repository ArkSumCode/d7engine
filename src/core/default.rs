/*
the default trait is used to create empty
data
we need that because we load data afterwards
*/
pub trait Default {
    fn default() -> Self;
}