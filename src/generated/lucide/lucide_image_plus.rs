use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h7" ></ path > < line y1 = "5" x1 = "16" x2 = "22" y2 = "5" ></ line > < line x1 = "19" y2 = "8" y1 = "2" x2 = "19" ></ line > < circle cy = "9" r = "2" cx = "9" ></ circle > < path d = "m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21" ></ path > < / > } } pub const LUCIDE_IMAGE_PLUS : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;