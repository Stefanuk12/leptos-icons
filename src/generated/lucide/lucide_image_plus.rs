use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 5h6" ></ path > < path d = "M19 2v6" ></ path > < path d = "M21 11.5V19a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h7.5" ></ path > < path d = "m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21" ></ path > < circle cy = "9" r = "2" cx = "9" ></ circle > < / > } } pub const LUCIDE_IMAGE_PLUS : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2")] } ;