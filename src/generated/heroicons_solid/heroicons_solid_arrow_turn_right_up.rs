use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path fill - rule = "evenodd" d = "M3.738 20.249a.75.75 0 0 1 .75-.75H14.99V5.56l-2.47 2.47a.75.75 0 0 1-1.06-1.061l3.75-3.75a.75.75 0 0 1 1.06 0l3.751 3.75a.75.75 0 0 1-1.06 1.06L16.49 5.56V20.25a.75.75 0 0 1-.75.75H4.487a.75.75 0 0 1-.75-.75Z" clip - rule = "evenodd" ></ path > < / > } } pub const HEROICONS_SOLID_ARROW_TURN_RIGHT_UP : Path = Path { path : icon_path , props : & [("fill" , "currentColor") , ("aria-hidden" , "true") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("data-slot" , "icon")] } ;