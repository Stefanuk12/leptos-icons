use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" stroke - linecap = "round" stroke - linejoin = "round" ></ path > < path stroke - linecap = "round" d = "M15.91 11.672a.375.375 0 0 1 0 .656l-5.603 3.113a.375.375 0 0 1-.557-.328V8.887c0-.286.307-.466.557-.327l5.603 3.112Z" stroke - linejoin = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_PLAY_CIRCLE : Path = Path { path : icon_path , props : & [("data-slot" , "icon") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "1.5") , ("stroke" , "currentColor") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("aria-hidden" , "true")] } ;