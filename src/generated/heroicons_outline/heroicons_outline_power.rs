use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "M5.636 5.636a9 9 0 1 0 12.728 0M12 3v9" ></ path > < / > } } pub const HEROICONS_OUTLINE_POWER : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("aria-hidden" , "true") , ("stroke-width" , "1.5") , ("data-slot" , "icon") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor")] } ;