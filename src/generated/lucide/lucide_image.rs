use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect ry = "2" height = "18" rx = "2" width = "18" y = "3" x = "3" ></ rect > < circle cx = "9" cy = "9" r = "2" ></ circle > < path d = "m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21" ></ path > < / > } } pub const LUCIDE_IMAGE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24")] } ;