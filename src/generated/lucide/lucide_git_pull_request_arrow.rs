use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "6" r = "3" cx = "5" ></ circle > < path d = "M5 9v12" ></ path > < circle r = "3" cx = "19" cy = "18" ></ circle > < path d = "m15 9-3-3 3-3" ></ path > < path d = "M12 6h5a2 2 0 0 1 2 2v7" ></ path > < / > } } pub const LUCIDE_GIT_PULL_REQUEST_ARROW : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("width" , "24") , ("stroke-width" , "2")] } ;