use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 12h8" ></ path > < path d = "M4 18V6" ></ path > < path d = "M12 18V6" ></ path > < circle cx = "19" r = "2" cy = "16" ></ circle > < path d = "M20 10c-2 2-3 3.5-3 6" ></ path > < / > } } pub const LUCIDE_HEADING_6 : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;