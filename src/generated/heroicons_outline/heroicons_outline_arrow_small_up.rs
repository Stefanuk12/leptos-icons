use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 19.5v-15m0 0-6.75 6.75M12 4.5l6.75 6.75" stroke - linejoin = "round" stroke - linecap = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_ARROW_SMALL_UP : Path = Path { path : icon_path , props : & [("fill" , "none") , ("viewBox" , "0 0 24 24") , ("aria-hidden" , "true") , ("stroke" , "currentColor") , ("stroke-width" , "1.5") , ("data-slot" , "icon") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;