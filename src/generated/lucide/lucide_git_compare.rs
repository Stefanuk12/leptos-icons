use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "18" cy = "18" r = "3" ></ circle > < circle cx = "6" cy = "6" r = "3" ></ circle > < path d = "M13 6h3a2 2 0 0 1 2 2v7" ></ path > < path d = "M11 18H8a2 2 0 0 1-2-2V9" ></ path > < / > } } pub const LUCIDE_GIT_COMPARE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none")] } ;