use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "18" cy = "18" r = "3" ></ circle > < circle r = "3" cy = "6" cx = "6" ></ circle > < path d = "M13 6h3a2 2 0 0 1 2 2v7" ></ path > < line x1 = "6" y2 = "21" x2 = "6" y1 = "9" ></ line > < / > } } pub const LUCIDE_GIT_PULL_REQUEST : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor")] } ;