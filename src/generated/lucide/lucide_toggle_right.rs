use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "6" rx = "6" ry = "6" height = "12" x = "2" width = "20" ></ rect > < circle cx = "16" r = "2" cy = "12" ></ circle > < / > } } pub const LUCIDE_TOGGLE_RIGHT : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;