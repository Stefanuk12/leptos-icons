use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "2" height = "12" rx = "6" ry = "6" y = "6" width = "20" ></ rect > < circle r = "2" cx = "8" cy = "12" ></ circle > < / > } } pub const LUCIDE_TOGGLE_LEFT : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("width" , "24")] } ;