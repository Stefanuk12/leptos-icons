use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "6" x1 = "22" x2 = "2" y1 = "6" ></ line > < line x1 = "22" x2 = "2" y2 = "18" y1 = "18" ></ line > < line y1 = "2" x2 = "6" x1 = "6" y2 = "22" ></ line > < line x1 = "18" y2 = "22" y1 = "2" x2 = "18" ></ line > < / > } } pub const LUCIDE_FRAME : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;