use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "10" cy = "7" r = "4" ></ circle > < path d = "M10.3 15H7a4 4 0 0 0-4 4v2" ></ path > < circle r = "3" cx = "17" cy = "17" ></ circle > < path d = "m21 21-1.9-1.9" ></ path > < / > } } pub const LUCIDE_USER_SEARCH : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("height" , "24")] } ;