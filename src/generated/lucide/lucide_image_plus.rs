use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h7" ></ path > < line x2 = "22" x1 = "16" y1 = "5" y2 = "5" ></ line > < line x1 = "19" x2 = "19" y1 = "2" y2 = "8" ></ line > < circle cx = "9" cy = "9" r = "2" ></ circle > < path d = "m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21" ></ path > < / > } } pub const LUCIDE_IMAGE_PLUS : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24")] } ;