use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "5" cy = "6" r = "3" ></ circle > < path d = "M5 9v12" ></ path > < path d = "m15 9-3-3 3-3" ></ path > < path d = "M12 6h5a2 2 0 0 1 2 2v3" ></ path > < path d = "M19 15v6" ></ path > < path d = "M22 18h-6" ></ path > < / > } } pub const LUCIDE_GIT_PULL_REQUEST_CREATE_ARROW : Path = Path { path : icon_path , props : & [("fill" , "none") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;