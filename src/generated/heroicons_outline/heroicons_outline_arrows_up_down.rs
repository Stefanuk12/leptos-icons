use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" d = "M3 7.5 7.5 3m0 0L12 7.5M7.5 3v13.5m13.5 0L16.5 21m0 0L12 16.5m4.5 4.5V7.5" stroke - linecap = "round" ></ path > < / > } } pub const HeroiconsOutlineArrowsUpDown : Path = Path { path : icon_path , props : & [("data-slot" , "icon") , ("stroke-width" , "1.5") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("aria-hidden" , "true") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;