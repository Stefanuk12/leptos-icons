use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "6" cx = "5" r = "3" ></ circle > < path d = "M5 9v12" ></ path > < path d = "m15 9-3-3 3-3" ></ path > < path d = "M12 6h5a2 2 0 0 1 2 2v3" ></ path > < path d = "M19 15v6" ></ path > < path d = "M22 18h-6" ></ path > < / > } } pub const LUCIDE_GIT_PULL_REQUEST_CREATE_ARROW : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor")] } ;