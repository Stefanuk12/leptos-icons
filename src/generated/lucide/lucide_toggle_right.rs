use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "6" rx = "6" ry = "6" x = "2" width = "20" height = "12" ></ rect > < circle r = "2" cy = "12" cx = "16" ></ circle > < / > } } pub const LUCIDE_TOGGLE_RIGHT : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor")] } ;