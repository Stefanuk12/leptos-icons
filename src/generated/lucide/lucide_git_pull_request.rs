use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "18" r = "3" cx = "18" ></ circle > < circle cy = "6" r = "3" cx = "6" ></ circle > < path d = "M13 6h3a2 2 0 0 1 2 2v7" ></ path > < line y2 = "21" x2 = "6" y1 = "9" x1 = "6" ></ line > < / > } } pub const LUCIDE_GIT_PULL_REQUEST : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;