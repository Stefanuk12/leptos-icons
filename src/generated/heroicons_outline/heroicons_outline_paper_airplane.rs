use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" d = "M6 12 3.269 3.125A59.769 59.769 0 0 1 21.485 12 59.768 59.768 0 0 1 3.27 20.875L5.999 12Zm0 0h7.5" stroke - linecap = "round" ></ path > < / > } } pub const HeroiconsOutlinePaperAirplane : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("data-slot" , "icon") , ("viewBox" , "0 0 24 24") , ("aria-hidden" , "true") , ("stroke-width" , "1.5") , ("stroke" , "currentColor")] } ;