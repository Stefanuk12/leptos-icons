use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 12h8" ></ path > < path d = "M4 18V6" ></ path > < path d = "M12 18V6" ></ path > < circle r = "2" cy = "16" cx = "19" ></ circle > < path d = "M20 10c-2 2-3 3.5-3 6" ></ path > < / > } } pub const LUCIDE_HEADING_6 : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;