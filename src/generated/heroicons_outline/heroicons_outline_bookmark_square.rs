use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "M16.5 3.75V16.5L12 14.25 7.5 16.5V3.75m9 0H18A2.25 2.25 0 0 1 20.25 6v12A2.25 2.25 0 0 1 18 20.25H6A2.25 2.25 0 0 1 3.75 18V6A2.25 2.25 0 0 1 6 3.75h1.5m9 0h-9" ></ path > < / > } } pub const HeroiconsOutlineBookmarkSquare : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-width" , "1.5") , ("aria-hidden" , "true") , ("stroke" , "currentColor") , ("data-slot" , "icon") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;