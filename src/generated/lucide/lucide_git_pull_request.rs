use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "18" cx = "18" r = "3" ></ circle > < circle cx = "6" r = "3" cy = "6" ></ circle > < path d = "M13 6h3a2 2 0 0 1 2 2v7" ></ path > < line y1 = "9" x1 = "6" x2 = "6" y2 = "21" ></ line > < / > } } pub const LUCIDE_GIT_PULL_REQUEST : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2")] } ;