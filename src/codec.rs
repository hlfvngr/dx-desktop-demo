pub trait Codec {
    fn encode<'a, 'b: 'a>(&'a mut self, data: impl Into<&'b str>) -> anyhow::Result<String>;
    fn decode<'a, 'b: 'a>(&'a mut self, data: impl Into<&'b str>) -> anyhow::Result<String>;
}
