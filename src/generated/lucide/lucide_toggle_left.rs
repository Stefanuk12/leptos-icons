use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect ry = "6" height = "12" width = "20" rx = "6" x = "2" y = "6" ></ rect > < circle r = "2" cy = "12" cx = "8" ></ circle > < / > } } pub const LUCIDE_TOGGLE_LEFT : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round")] } ;