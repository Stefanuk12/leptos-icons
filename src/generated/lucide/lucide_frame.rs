use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "6" x2 = "2" x1 = "22" y2 = "6" ></ line > < line y2 = "18" y1 = "18" x2 = "2" x1 = "22" ></ line > < line y1 = "2" x2 = "6" x1 = "6" y2 = "22" ></ line > < line x1 = "18" y1 = "2" x2 = "18" y2 = "22" ></ line > < / > } } pub const LUCIDE_FRAME : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2")] } ;