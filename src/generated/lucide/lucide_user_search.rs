use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "7" cx = "10" r = "4" ></ circle > < path d = "M10.3 15H7a4 4 0 0 0-4 4v2" ></ path > < circle cx = "17" r = "3" cy = "17" ></ circle > < path d = "m21 21-1.9-1.9" ></ path > < / > } } pub const LUCIDE_USER_SEARCH : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("width" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2")] } ;