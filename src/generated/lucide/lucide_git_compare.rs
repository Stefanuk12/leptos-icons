use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "18" cx = "18" r = "3" ></ circle > < circle cy = "6" r = "3" cx = "6" ></ circle > < path d = "M13 6h3a2 2 0 0 1 2 2v7" ></ path > < path d = "M11 18H8a2 2 0 0 1-2-2V9" ></ path > < / > } } pub const LUCIDE_GIT_COMPARE : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-width" , "2")] } ;