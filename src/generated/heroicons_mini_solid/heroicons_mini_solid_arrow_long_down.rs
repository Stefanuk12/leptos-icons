use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path fill - rule = "evenodd" d = "M10 2a.75.75 0 0 1 .75.75v12.59l1.95-2.1a.75.75 0 1 1 1.1 1.02l-3.25 3.5a.75.75 0 0 1-1.1 0l-3.25-3.5a.75.75 0 1 1 1.1-1.02l1.95 2.1V2.75A.75.75 0 0 1 10 2Z" clip - rule = "evenodd" ></ path > < / > } } pub const HEROICONS_MINI_SOLID_ARROW_LONG_DOWN : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("aria-hidden" , "true") , ("viewBox" , "0 0 20 20") , ("fill" , "currentColor") , ("data-slot" , "icon")] } ;