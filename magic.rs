#[derive(Serialize, Deserialize, Clone, Debug)] pub enum ComponentEnum
{
    Component0(Set),
    Component1(quux :: initialisation_script :: InitialisationScript <
    ComponentEnum >)
} impl quux :: component :: Enum for ComponentEnum
{
    fn render(self, context : render :: Context < Self >) -> quux :: component
    :: EnumRenderOutput < Self >
    {
        match self
        {
            Self :: Component0(component) => component.render(context).into(),
            Self :: Component1(component) => component.render(context).into()
        }
    }
} impl From < Set > for ComponentEnum
{ fn from(value : Set) -> Self { Self :: Component0(value) } } impl TryFrom <
ComponentEnum > for Set
{
    type Error = () ; fn try_from(value : ComponentEnum) -> Result < Self,
    Self :: Error >
    {
        if let ComponentEnum :: Component0(component) = value
        { Ok(component) } else { Err(()) }
    }
} impl From < quux :: initialisation_script :: InitialisationScript <
ComponentEnum > > for ComponentEnum
{
    fn
    from(value : quux :: initialisation_script :: InitialisationScript <
    ComponentEnum >) -> Self { Self :: Component1(value) }
} impl TryFrom < ComponentEnum > for quux :: initialisation_script ::
InitialisationScript < ComponentEnum >
{
    type Error = () ; fn try_from(value : ComponentEnum) -> Result < Self,
    Self :: Error >
    {
        if let ComponentEnum :: Component1(component) = value
        { Ok(component) } else { Err(()) }
    }
}