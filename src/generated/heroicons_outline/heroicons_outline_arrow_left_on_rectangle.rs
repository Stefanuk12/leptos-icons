use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" d = "M15.75 9V5.25A2.25 2.25 0 0 0 13.5 3h-6a2.25 2.25 0 0 0-2.25 2.25v13.5A2.25 2.25 0 0 0 7.5 21h6a2.25 2.25 0 0 0 2.25-2.25V15M12 9l-3 3m0 0 3 3m-3-3h12.75" stroke - linecap = "round" ></ path > < / > } } pub const HeroiconsOutlineArrowLeftOnRectangle : Path = Path { path : icon_path , props : & [("aria-hidden" , "true") , ("stroke-width" , "1.5") , ("fill" , "none") , ("data-slot" , "icon") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;