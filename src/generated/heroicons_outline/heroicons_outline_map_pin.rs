use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" d = "M15 10.5a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z" stroke - linejoin = "round" ></ path > < path stroke - linecap = "round" d = "M19.5 10.5c0 7.142-7.5 11.25-7.5 11.25S4.5 17.642 4.5 10.5a7.5 7.5 0 1 1 15 0Z" stroke - linejoin = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_MAP_PIN : Path = Path { path : icon_path , props : & [("stroke-width" , "1.5") , ("viewBox" , "0 0 24 24") , ("aria-hidden" , "true") , ("data-slot" , "icon") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("fill" , "none")] } ;