use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" d = "m8.25 4.5 7.5 7.5-7.5 7.5" stroke - linecap = "round" ></ path > < / > } } pub const HeroiconsOutlineChevronRight : Path = Path { path : icon_path , props : & [("aria-hidden" , "true") , ("stroke" , "currentColor") , ("data-slot" , "icon") , ("fill" , "none") , ("stroke-width" , "1.5") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;