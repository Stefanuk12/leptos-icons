use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "m9 9 6-6m0 0 6 6m-6-6v12a6 6 0 0 1-12 0v-3" ></ path > < / > } } pub const HEROICONS_OUTLINE_ARROW_UTURN_UP : Path = Path { path : icon_path , props : & [("stroke-width" , "1.5") , ("stroke" , "currentColor") , ("aria-hidden" , "true") , ("viewBox" , "0 0 24 24") , ("data-slot" , "icon") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;