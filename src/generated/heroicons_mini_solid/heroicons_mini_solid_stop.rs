use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M5.25 3A2.25 2.25 0 0 0 3 5.25v9.5A2.25 2.25 0 0 0 5.25 17h9.5A2.25 2.25 0 0 0 17 14.75v-9.5A2.25 2.25 0 0 0 14.75 3h-9.5Z" ></ path > < / > } } pub const HEROICONS_MINI_SOLID_STOP : Path = Path { path : icon_path , props : & [("fill" , "currentColor") , ("aria-hidden" , "true") , ("xmlns" , "http://www.w3.org/2000/svg") , ("data-slot" , "icon") , ("viewBox" , "0 0 20 20")] } ;