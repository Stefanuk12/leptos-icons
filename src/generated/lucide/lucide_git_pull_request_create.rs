use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "6" r = "3" cx = "6" ></ circle > < path d = "M6 9v12" ></ path > < path d = "M13 6h3a2 2 0 0 1 2 2v3" ></ path > < path d = "M18 15v6" ></ path > < path d = "M21 18h-6" ></ path > < / > } } pub const LUCIDE_GIT_PULL_REQUEST_CREATE : Path = Path { path : icon_path , props : & [("height" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;