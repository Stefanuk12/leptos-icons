use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m12 8-4 4 4 4" ></ path > < path d = "M16 12H8" ></ path > < / > } } pub const LucideArrowLeftSquare : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;