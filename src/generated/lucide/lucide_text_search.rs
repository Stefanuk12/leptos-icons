use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 6H3" ></ path > < path d = "M10 12H3" ></ path > < path d = "M10 18H3" ></ path > < circle r = "3" cx = "17" cy = "15" ></ circle > < path d = "m21 19-1.9-1.9" ></ path > < / > } } pub const LUCIDE_TEXT_SEARCH : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;