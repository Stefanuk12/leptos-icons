use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M15 14h.01" ></ path > < path d = "M9 6h6" ></ path > < path d = "M9 10h6" ></ path > < / > } } pub const LucidePcCase : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("height" , "24")] } ;