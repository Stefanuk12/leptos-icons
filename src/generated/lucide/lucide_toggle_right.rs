use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "12" rx = "6" ry = "6" y = "6" width = "20" x = "2" ></ rect > < circle cx = "16" cy = "12" r = "2" ></ circle > < / > } } pub const LUCIDE_TOGGLE_RIGHT : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("height" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;