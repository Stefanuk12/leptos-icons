use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 8.25H7.5a2.25 2.25 0 0 0-2.25 2.25v9a2.25 2.25 0 0 0 2.25 2.25h9a2.25 2.25 0 0 0 2.25-2.25v-9a2.25 2.25 0 0 0-2.25-2.25H15m0-3-3-3m0 0-3 3m3-3V15" stroke - linecap = "round" stroke - linejoin = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_ARROW_UP_ON_SQUARE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-width" , "1.5") , ("viewBox" , "0 0 24 24") , ("aria-hidden" , "true") , ("fill" , "none") , ("data-slot" , "icon") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;