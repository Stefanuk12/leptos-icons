use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" d = "M12 6v12m6-6H6" stroke - linecap = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_PLUS_SMALL : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "1.5") , ("stroke" , "currentColor") , ("data-slot" , "icon") , ("aria-hidden" , "true")] } ;