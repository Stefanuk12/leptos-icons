use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "12" y = "6" x = "2" rx = "6" ry = "6" width = "20" ></ rect > < circle cy = "12" cx = "8" r = "2" ></ circle > < / > } } pub const LUCIDE_TOGGLE_LEFT : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24")] } ;