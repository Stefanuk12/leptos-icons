use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 6h1v4" ></ path > < path d = "M4 10h2" ></ path > < path d = "M6 18H4c0-1 2-2 2-3s-1-1.5-2-1" ></ path > < / > } } pub const LucideListOrdered : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;