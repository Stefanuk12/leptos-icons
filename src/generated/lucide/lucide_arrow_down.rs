use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 5v14" ></ path > < path d = "m19 12-7 7-7-7" ></ path > < / > } } pub const LucideArrowDown : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;