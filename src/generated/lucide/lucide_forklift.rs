use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 12H5a2 2 0 0 0-2 2v5" ></ path > < circle r = "2" cy = "19" cx = "13" ></ circle > < circle cx = "5" cy = "19" r = "2" ></ circle > < path d = "M8 19h3m5-17v17h6M6 12V7c0-1.1.9-2 2-2h3l5 5" ></ path > < / > } } pub const LucideForklift : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("height" , "24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;