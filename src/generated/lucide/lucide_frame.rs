use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "6" x2 = "2" x1 = "22" y1 = "6" ></ line > < line x2 = "2" x1 = "22" y2 = "18" y1 = "18" ></ line > < line x1 = "6" y2 = "22" x2 = "6" y1 = "2" ></ line > < line x1 = "18" y1 = "2" y2 = "22" x2 = "18" ></ line > < / > } } pub const LUCIDE_FRAME : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2")] } ;