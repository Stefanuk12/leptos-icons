use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "6" height = "12" width = "20" x = "2" y = "6" ry = "6" ></ rect > < circle r = "2" cx = "16" cy = "12" ></ circle > < / > } } pub const LUCIDE_TOGGLE_RIGHT : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2")] } ;