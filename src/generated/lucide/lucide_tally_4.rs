use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 4v16" ></ path > < path d = "M9 4v16" ></ path > < path d = "M14 4v16" ></ path > < path d = "M19 4v16" ></ path > < / > } } pub const LucideTally4 : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;