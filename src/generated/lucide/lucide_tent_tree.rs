use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m14 5 3-3 3 3" ></ path > < path d = "m14 10 3-3 3 3" ></ path > < path d = "M17 14V2" ></ path > < path d = "M17 14H7l-5 8h20Z" ></ path > < path d = "M8 14v8" ></ path > < path d = "m9 14 5 8" ></ path > < / > } } pub const LucideTentTree : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("fill" , "none")] } ;