use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 12H5a2 2 0 0 0-2 2v5" ></ path > < circle cy = "19" r = "2" cx = "13" ></ circle > < circle r = "2" cy = "19" cx = "5" ></ circle > < path d = "M8 19h3m5-17v17h6M6 12V7c0-1.1.9-2 2-2h3l5 5" ></ path > < / > } } pub const LUCIDE_FORKLIFT : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("stroke" , "currentColor")] } ;