use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" stroke - linecap = "round" d = "m9 7.5 3 4.5m0 0 3-4.5M12 12v5.25M15 12H9m6 3H9m12-3a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" ></ path > < / > } } pub const HEROICONS_OUTLINE_CURRENCY_YEN : Path = Path { path : icon_path , props : & [("data-slot" , "icon") , ("stroke-width" , "1.5") , ("fill" , "none") , ("aria-hidden" , "true") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;