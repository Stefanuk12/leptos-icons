use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "3" cx = "18" cy = "18" ></ circle > < circle cx = "6" cy = "6" r = "3" ></ circle > < path d = "M13 6h3a2 2 0 0 1 2 2v7" ></ path > < line x1 = "6" y2 = "21" y1 = "9" x2 = "6" ></ line > < / > } } pub const LUCIDE_GIT_PULL_REQUEST : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24")] } ;