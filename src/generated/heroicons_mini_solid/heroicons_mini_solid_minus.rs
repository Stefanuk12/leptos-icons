use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path clip - rule = "evenodd" d = "M4 10a.75.75 0 0 1 .75-.75h10.5a.75.75 0 0 1 0 1.5H4.75A.75.75 0 0 1 4 10Z" fill - rule = "evenodd" ></ path > < / > } } pub const HEROICONS_MINI_SOLID_MINUS : Path = Path { path : icon_path , props : & [("fill" , "currentColor") , ("data-slot" , "icon") , ("aria-hidden" , "true") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 20 20")] } ;