use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" d = "M5.25 8.25h15m-16.5 7.5h15m-1.8-13.5-3.9 19.5m-2.1-19.5-3.9 19.5" stroke - linecap = "round" ></ path > < / > } } pub const HeroiconsOutlineHashtag : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("aria-hidden" , "true") , ("stroke-width" , "1.5") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("data-slot" , "icon")] } ;