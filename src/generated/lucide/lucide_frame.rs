use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "22" y1 = "6" x2 = "2" y2 = "6" ></ line > < line y1 = "18" x1 = "22" x2 = "2" y2 = "18" ></ line > < line x1 = "6" x2 = "6" y2 = "22" y1 = "2" ></ line > < line x2 = "18" x1 = "18" y2 = "22" y1 = "2" ></ line > < / > } } pub const LUCIDE_FRAME : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24")] } ;