use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "2" x1 = "22" y1 = "6" y2 = "6" ></ line > < line x2 = "2" y1 = "18" x1 = "22" y2 = "18" ></ line > < line x1 = "6" y2 = "22" y1 = "2" x2 = "6" ></ line > < line y2 = "22" x1 = "18" y1 = "2" x2 = "18" ></ line > < / > } } pub const LUCIDE_FRAME : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;