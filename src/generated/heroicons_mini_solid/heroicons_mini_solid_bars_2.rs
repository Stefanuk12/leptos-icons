use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path fill - rule = "evenodd" d = "M2 6.75A.75.75 0 0 1 2.75 6h14.5a.75.75 0 0 1 0 1.5H2.75A.75.75 0 0 1 2 6.75Zm0 6.5a.75.75 0 0 1 .75-.75h14.5a.75.75 0 0 1 0 1.5H2.75a.75.75 0 0 1-.75-.75Z" clip - rule = "evenodd" ></ path > < / > } } pub const HEROICONS_MINI_SOLID_BARS_2 : Path = Path { path : icon_path , props : & [("aria-hidden" , "true") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "currentColor") , ("data-slot" , "icon") , ("viewBox" , "0 0 20 20")] } ;