use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "m5.25 4.5 7.5 7.5-7.5 7.5m6-15 7.5 7.5-7.5 7.5" ></ path > < / > } } pub const HeroiconsOutlineChevronDoubleRight : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("aria-hidden" , "true") , ("stroke-width" , "1.5") , ("fill" , "none") , ("data-slot" , "icon") , ("viewBox" , "0 0 24 24")] } ;