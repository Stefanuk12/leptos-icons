use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m15 15 6-6m0 0-6-6m6 6H9a6 6 0 0 0 0 12h3" stroke - linejoin = "round" stroke - linecap = "round" ></ path > < / > } } pub const HeroiconsOutlineArrowUturnRight : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("aria-hidden" , "true") , ("data-slot" , "icon") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "1.5")] } ;