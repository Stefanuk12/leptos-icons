use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10.75 4.75a.75.75 0 0 0-1.5 0v4.5h-4.5a.75.75 0 0 0 0 1.5h4.5v4.5a.75.75 0 0 0 1.5 0v-4.5h4.5a.75.75 0 0 0 0-1.5h-4.5v-4.5Z" ></ path > < / > } } pub const HEROICONS_MINI_SOLID_PLUS : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 20 20") , ("aria-hidden" , "true") , ("xmlns" , "http://www.w3.org/2000/svg") , ("data-slot" , "icon") , ("fill" , "currentColor")] } ;