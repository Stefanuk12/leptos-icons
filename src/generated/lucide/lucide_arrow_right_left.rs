use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m16 3 4 4-4 4" ></ path > < path d = "M20 7H4" ></ path > < path d = "m8 21-4-4 4-4" ></ path > < path d = "M4 17h16" ></ path > < / > } } pub const LucideArrowRightLeft : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;