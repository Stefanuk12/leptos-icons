use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10.5 6a1.5 1.5 0 1 1 3 0 1.5 1.5 0 0 1-3 0Zm0 6a1.5 1.5 0 1 1 3 0 1.5 1.5 0 0 1-3 0Zm0 6a1.5 1.5 0 1 1 3 0 1.5 1.5 0 0 1-3 0Z" clip - rule = "evenodd" fill - rule = "evenodd" ></ path > < / > } } pub const HEROICONS_SOLID_ELLIPSIS_VERTICAL : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "currentColor") , ("aria-hidden" , "true") , ("viewBox" , "0 0 24 24") , ("data-slot" , "icon")] } ;