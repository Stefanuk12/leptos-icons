use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 12H5a2 2 0 0 0-2 2v5" ></ path > < circle cx = "13" cy = "19" r = "2" ></ circle > < circle cy = "19" cx = "5" r = "2" ></ circle > < path d = "M8 19h3m5-17v17h6M6 12V7c0-1.1.9-2 2-2h3l5 5" ></ path > < / > } } pub const LUCIDE_FORKLIFT : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;