use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "3" cy = "6" cx = "5" ></ circle > < path d = "M5 9v6" ></ path > < circle cy = "18" cx = "5" r = "3" ></ circle > < path d = "M12 3v18" ></ path > < circle cx = "19" cy = "6" r = "3" ></ circle > < path d = "M16 15.7A9 9 0 0 0 19 9" ></ path > < / > } } pub const LUCIDE_GIT_GRAPH : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;