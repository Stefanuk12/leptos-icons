use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path fill - rule = "evenodd" d = "M3 16.25a.75.75 0 0 1 .75-.75h7.5V4.56L9.28 6.53a.75.75 0 0 1-1.06-1.06l3.25-3.25a.75.75 0 0 1 1.06 0l3.25 3.25a.75.75 0 0 1-1.06 1.06l-1.97-1.97v11.69A.75.75 0 0 1 12 17H3.75a.75.75 0 0 1-.75-.75Z" clip - rule = "evenodd" ></ path > < / > } } pub const HEROICONS_MINI_SOLID_ARROW_TURN_RIGHT_UP : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 20 20") , ("data-slot" , "icon") , ("xmlns" , "http://www.w3.org/2000/svg") , ("aria-hidden" , "true") , ("fill" , "currentColor")] } ;