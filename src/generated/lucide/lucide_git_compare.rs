use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "3" cx = "18" cy = "18" ></ circle > < circle cy = "6" cx = "6" r = "3" ></ circle > < path d = "M13 6h3a2 2 0 0 1 2 2v7" ></ path > < path d = "M11 18H8a2 2 0 0 1-2-2V9" ></ path > < / > } } pub const LUCIDE_GIT_COMPARE : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24")] } ;