use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3.5 13h6" ></ path > < path d = "m2 16 4.5-9 4.5 9" ></ path > < path d = "M18 16V7" ></ path > < path d = "m14 11 4-4 4 4" ></ path > < / > } } pub const LucideAArrowUp : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24") , ("stroke" , "currentColor")] } ;