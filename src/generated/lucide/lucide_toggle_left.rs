use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "6" ry = "6" height = "12" width = "20" x = "2" y = "6" ></ rect > < circle cx = "8" cy = "12" r = "2" ></ circle > < / > } } pub const LUCIDE_TOGGLE_LEFT : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("fill" , "none") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("height" , "24")] } ;