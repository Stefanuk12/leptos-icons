use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path fill - rule = "evenodd" clip - rule = "evenodd" d = "M10 15a.75.75 0 0 1-.75-.75V7.612L7.29 9.77a.75.75 0 0 1-1.08-1.04l3.25-3.5a.75.75 0 0 1 1.08 0l3.25 3.5a.75.75 0 1 1-1.08 1.04l-1.96-2.158v6.638A.75.75 0 0 1 10 15Z" ></ path > < / > } } pub const HEROICONS_MINI_SOLID_ARROW_SMALL_UP : Path = Path { path : icon_path , props : & [("data-slot" , "icon") , ("viewBox" , "0 0 20 20") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "currentColor") , ("aria-hidden" , "true")] } ;