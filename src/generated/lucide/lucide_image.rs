use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" y = "3" ry = "2" rx = "2" x = "3" height = "18" ></ rect > < circle cx = "9" cy = "9" r = "2" ></ circle > < path d = "m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21" ></ path > < / > } } pub const LUCIDE_IMAGE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;