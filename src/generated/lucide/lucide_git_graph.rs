use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "5" cy = "6" r = "3" ></ circle > < path d = "M5 9v6" ></ path > < circle cx = "5" r = "3" cy = "18" ></ circle > < path d = "M12 3v18" ></ path > < circle cy = "6" cx = "19" r = "3" ></ circle > < path d = "M16 15.7A9 9 0 0 0 19 9" ></ path > < / > } } pub const LUCIDE_GIT_GRAPH : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke" , "currentColor")] } ;