use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3.5 21 14 3" ></ path > < path d = "M20.5 21 10 3" ></ path > < path d = "M15.5 21 12 15l-3.5 6" ></ path > < path d = "M2 21h20" ></ path > < / > } } pub const LucideTent : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;