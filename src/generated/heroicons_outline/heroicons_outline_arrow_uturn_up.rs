use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "m9 9 6-6m0 0 6 6m-6-6v12a6 6 0 0 1-12 0v-3" ></ path > < / > } } pub const HEROICONS_OUTLINE_ARROW_UTURN_UP : Path = Path { path : icon_path , props : & [("data-slot" , "icon") , ("aria-hidden" , "true") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "1.5") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor")] } ;