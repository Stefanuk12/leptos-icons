use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" stroke - linecap = "round" d = "M12.75 19.5v-.75a7.5 7.5 0 0 0-7.5-7.5H4.5m0-6.75h.75c7.87 0 14.25 6.38 14.25 14.25v.75M6 18.75a.75.75 0 1 1-1.5 0 .75.75 0 0 1 1.5 0Z" ></ path > < / > } } pub const HEROICONS_OUTLINE_RSS : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "1.5") , ("xmlns" , "http://www.w3.org/2000/svg") , ("aria-hidden" , "true") , ("data-slot" , "icon") , ("stroke" , "currentColor") , ("fill" , "none")] } ;