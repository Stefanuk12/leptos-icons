use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3.75 6a.75.75 0 0 0 0 1.5h12.5a.75.75 0 0 0 0-1.5H3.75ZM3.75 13.5a.75.75 0 0 0 0 1.5h12.5a.75.75 0 0 0 0-1.5H3.75Z" ></ path > < / > } } pub const HEROICONS_MINI_SOLID_EQUALS : Path = Path { path : icon_path , props : & [("data-slot" , "icon") , ("fill" , "currentColor") , ("aria-hidden" , "true") , ("viewBox" , "0 0 20 20") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;