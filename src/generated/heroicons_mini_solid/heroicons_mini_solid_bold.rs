use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 3a1 1 0 0 1 1-1h6a4.5 4.5 0 0 1 3.274 7.587A4.75 4.75 0 0 1 11.25 18H5a1 1 0 0 1-1-1V3Zm2.5 5.5v-4H11a2 2 0 1 1 0 4H6.5Zm0 2.5v4.5h4.75a2.25 2.25 0 0 0 0-4.5H6.5Z" clip - rule = "evenodd" fill - rule = "evenodd" ></ path > < / > } } pub const HEROICONS_MINI_SOLID_BOLD : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "currentColor") , ("viewBox" , "0 0 20 20") , ("aria-hidden" , "true") , ("data-slot" , "icon")] } ;