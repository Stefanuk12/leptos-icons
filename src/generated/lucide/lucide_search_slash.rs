use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m13.5 8.5-5 5" ></ path > < circle r = "8" cx = "11" cy = "11" ></ circle > < path d = "m21 21-4.3-4.3" ></ path > < / > } } pub const LucideSearchSlash : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24")] } ;