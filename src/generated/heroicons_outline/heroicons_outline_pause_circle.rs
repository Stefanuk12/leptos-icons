use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" d = "M14.25 9v6m-4.5 0V9M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" stroke - linecap = "round" ></ path > < / > } } pub const HeroiconsOutlinePauseCircle : Path = Path { path : icon_path , props : & [("aria-hidden" , "true") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("data-slot" , "icon") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "1.5")] } ;