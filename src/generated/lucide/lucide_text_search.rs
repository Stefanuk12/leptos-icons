use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 6H3" ></ path > < path d = "M10 12H3" ></ path > < path d = "M10 18H3" ></ path > < circle cy = "15" r = "3" cx = "17" ></ circle > < path d = "m21 19-1.9-1.9" ></ path > < / > } } pub const LUCIDE_TEXT_SEARCH : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("width" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;