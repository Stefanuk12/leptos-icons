use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3.5 13h6" ></ path > < path d = "m2 16 4.5-9 4.5 9" ></ path > < path d = "M18 7v9" ></ path > < path d = "m14 12 4 4 4-4" ></ path > < / > } } pub const LucideAArrowDown : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;