use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" d = "M12 4.5v15m7.5-7.5h-15" stroke - linecap = "round" ></ path > < / > } } pub const HeroiconsOutlinePlus : Path = Path { path : icon_path , props : & [("data-slot" , "icon") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "1.5") , ("stroke" , "currentColor") , ("aria-hidden" , "true") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;