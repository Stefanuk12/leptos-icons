use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "2" y1 = "6" y2 = "6" x1 = "22" ></ line > < line x1 = "22" x2 = "2" y1 = "18" y2 = "18" ></ line > < line y2 = "22" y1 = "2" x1 = "6" x2 = "6" ></ line > < line y1 = "2" x1 = "18" y2 = "22" x2 = "18" ></ line > < / > } } pub const LUCIDE_FRAME : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("fill" , "none")] } ;