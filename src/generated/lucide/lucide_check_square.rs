use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m9 11 3 3L22 4" ></ path > < path d = "M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11" ></ path > < / > } } pub const LucideCheckSquare : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24")] } ;