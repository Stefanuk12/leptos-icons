use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10.5 6a7.5 7.5 0 1 0 7.5 7.5h-7.5V6Z" stroke - linecap = "round" stroke - linejoin = "round" ></ path > < path d = "M13.5 10.5H21A7.5 7.5 0 0 0 13.5 3v7.5Z" stroke - linejoin = "round" stroke - linecap = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_CHART_PIE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("data-slot" , "icon") , ("fill" , "none") , ("stroke-width" , "1.5") , ("aria-hidden" , "true")] } ;