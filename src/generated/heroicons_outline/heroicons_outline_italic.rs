use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "M5.248 20.246H9.05m0 0h3.696m-3.696 0 5.893-16.502m0 0h-3.697m3.697 0h3.803" ></ path > < / > } } pub const HEROICONS_OUTLINE_ITALIC : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("aria-hidden" , "true") , ("data-slot" , "icon") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "1.5") , ("viewBox" , "0 0 24 24")] } ;