use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "M15 8.25H9m6 3H9m3 6-3-3h1.5a3 3 0 1 0 0-6M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" ></ path > < / > } } pub const HEROICONS_OUTLINE_CURRENCY_RUPEE : Path = Path { path : icon_path , props : & [("aria-hidden" , "true") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("data-slot" , "icon") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "1.5")] } ;