use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" d = "M10.5 6a7.5 7.5 0 1 0 7.5 7.5h-7.5V6Z" stroke - linejoin = "round" ></ path > < path stroke - linecap = "round" stroke - linejoin = "round" d = "M13.5 10.5H21A7.5 7.5 0 0 0 13.5 3v7.5Z" ></ path > < / > } } pub const HEROICONS_OUTLINE_CHART_PIE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("aria-hidden" , "true") , ("data-slot" , "icon") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "1.5")] } ;