use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M5.25 7.5A2.25 2.25 0 0 1 7.5 5.25h9a2.25 2.25 0 0 1 2.25 2.25v9a2.25 2.25 0 0 1-2.25 2.25h-9a2.25 2.25 0 0 1-2.25-2.25v-9Z" stroke - linejoin = "round" stroke - linecap = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_STOP : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("aria-hidden" , "true") , ("data-slot" , "icon") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "1.5")] } ;