use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m21 16-4 4-4-4" ></ path > < path d = "M17 20V4" ></ path > < path d = "m3 8 4-4 4 4" ></ path > < path d = "M7 4v16" ></ path > < / > } } pub const LucideArrowUpDown : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("width" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;