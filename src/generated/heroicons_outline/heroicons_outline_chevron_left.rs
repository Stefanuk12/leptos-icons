use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" d = "M15.75 19.5 8.25 12l7.5-7.5" stroke - linecap = "round" ></ path > < / > } } pub const HeroiconsOutlineChevronLeft : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "1.5") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("data-slot" , "icon") , ("aria-hidden" , "true")] } ;