use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect ry = "2" y = "3" width = "18" x = "3" height = "18" rx = "2" ></ rect > < circle r = "2" cx = "9" cy = "9" ></ circle > < path d = "m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21" ></ path > < / > } } pub const LUCIDE_IMAGE : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;