use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "3" cx = "6" cy = "6" ></ circle > < path d = "M6 9v12" ></ path > < path d = "M13 6h3a2 2 0 0 1 2 2v3" ></ path > < path d = "M18 15v6" ></ path > < path d = "M21 18h-6" ></ path > < / > } } pub const LUCIDE_GIT_PULL_REQUEST_CREATE : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;