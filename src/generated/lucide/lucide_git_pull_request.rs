use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "18" cy = "18" r = "3" ></ circle > < circle cy = "6" cx = "6" r = "3" ></ circle > < path d = "M13 6h3a2 2 0 0 1 2 2v7" ></ path > < line y2 = "21" y1 = "9" x2 = "6" x1 = "6" ></ line > < / > } } pub const LUCIDE_GIT_PULL_REQUEST : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("width" , "24")] } ;