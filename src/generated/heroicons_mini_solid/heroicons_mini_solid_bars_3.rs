use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path fill - rule = "evenodd" clip - rule = "evenodd" d = "M2 4.75A.75.75 0 0 1 2.75 4h14.5a.75.75 0 0 1 0 1.5H2.75A.75.75 0 0 1 2 4.75ZM2 10a.75.75 0 0 1 .75-.75h14.5a.75.75 0 0 1 0 1.5H2.75A.75.75 0 0 1 2 10Zm0 5.25a.75.75 0 0 1 .75-.75h14.5a.75.75 0 0 1 0 1.5H2.75a.75.75 0 0 1-.75-.75Z" ></ path > < / > } } pub const HEROICONS_MINI_SOLID_BARS_3 : Path = Path { path : icon_path , props : & [("data-slot" , "icon") , ("fill" , "currentColor") , ("viewBox" , "0 0 20 20") , ("aria-hidden" , "true") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;