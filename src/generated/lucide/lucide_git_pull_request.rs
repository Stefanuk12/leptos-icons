use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "3" cy = "18" cx = "18" ></ circle > < circle r = "3" cy = "6" cx = "6" ></ circle > < path d = "M13 6h3a2 2 0 0 1 2 2v7" ></ path > < line x2 = "6" y1 = "9" x1 = "6" y2 = "21" ></ line > < / > } } pub const LUCIDE_GIT_PULL_REQUEST : Path = Path { path : icon_path , props : & [("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;