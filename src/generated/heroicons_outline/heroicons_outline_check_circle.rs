use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 12.75 11.25 15 15 9.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" stroke - linecap = "round" stroke - linejoin = "round" ></ path > < / > } } pub const HeroiconsOutlineCheckCircle : Path = Path { path : icon_path , props : & [("data-slot" , "icon") , ("stroke-width" , "1.5") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("aria-hidden" , "true") , ("viewBox" , "0 0 24 24")] } ;