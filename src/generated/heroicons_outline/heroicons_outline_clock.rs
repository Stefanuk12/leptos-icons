use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 6v6h4.5m4.5 0a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" stroke - linejoin = "round" stroke - linecap = "round" ></ path > < / > } } pub const HeroiconsOutlineClock : Path = Path { path : icon_path , props : & [("aria-hidden" , "true") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "1.5") , ("xmlns" , "http://www.w3.org/2000/svg") , ("data-slot" , "icon")] } ;