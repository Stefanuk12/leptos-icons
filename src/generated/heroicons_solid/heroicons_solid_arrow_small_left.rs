use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path fill - rule = "evenodd" d = "M20.25 12a.75.75 0 0 1-.75.75H6.31l5.47 5.47a.75.75 0 1 1-1.06 1.06l-6.75-6.75a.75.75 0 0 1 0-1.06l6.75-6.75a.75.75 0 1 1 1.06 1.06l-5.47 5.47H19.5a.75.75 0 0 1 .75.75Z" clip - rule = "evenodd" ></ path > < / > } } pub const HEROICONS_SOLID_ARROW_SMALL_LEFT : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("aria-hidden" , "true") , ("fill" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("data-slot" , "icon")] } ;