use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" d = "M16.5 3.75V16.5L12 14.25 7.5 16.5V3.75m9 0H18A2.25 2.25 0 0 1 20.25 6v12A2.25 2.25 0 0 1 18 20.25H6A2.25 2.25 0 0 1 3.75 18V6A2.25 2.25 0 0 1 6 3.75h1.5m9 0h-9" stroke - linejoin = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_BOOKMARK_SQUARE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("data-slot" , "icon") , ("stroke-width" , "1.5") , ("aria-hidden" , "true") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;