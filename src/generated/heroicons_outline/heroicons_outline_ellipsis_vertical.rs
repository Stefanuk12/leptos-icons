use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 6.75a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5ZM12 12.75a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5ZM12 18.75a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5Z" stroke - linecap = "round" stroke - linejoin = "round" ></ path > < / > } } pub const HeroiconsOutlineEllipsisVertical : Path = Path { path : icon_path , props : & [("stroke-width" , "1.5") , ("stroke" , "currentColor") , ("aria-hidden" , "true") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("data-slot" , "icon") , ("viewBox" , "0 0 24 24")] } ;