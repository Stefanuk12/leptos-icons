use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "3" cy = "18" cx = "18" ></ circle > < circle r = "3" cx = "6" cy = "6" ></ circle > < path d = "M13 6h3a2 2 0 0 1 2 2v7" ></ path > < line y2 = "21" x1 = "6" x2 = "6" y1 = "9" ></ line > < / > } } pub const LUCIDE_GIT_PULL_REQUEST : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;