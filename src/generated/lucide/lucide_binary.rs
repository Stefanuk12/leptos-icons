use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M6 20h4" ></ path > < path d = "M14 10h4" ></ path > < path d = "M6 14h2v6" ></ path > < path d = "M14 4h2v6" ></ path > < / > } } pub const LucideBinary : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;