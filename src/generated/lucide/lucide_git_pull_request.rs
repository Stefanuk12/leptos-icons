use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "18" cx = "18" r = "3" ></ circle > < circle cx = "6" cy = "6" r = "3" ></ circle > < path d = "M13 6h3a2 2 0 0 1 2 2v7" ></ path > < line x2 = "6" x1 = "6" y1 = "9" y2 = "21" ></ line > < / > } } pub const LUCIDE_GIT_PULL_REQUEST : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;