use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "12" width = "20" rx = "6" ry = "6" x = "2" y = "6" ></ rect > < circle cx = "16" cy = "12" r = "2" ></ circle > < / > } } pub const LUCIDE_TOGGLE_RIGHT : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("height" , "24")] } ;