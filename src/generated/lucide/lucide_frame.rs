use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "6" x1 = "22" x2 = "2" y1 = "6" ></ line > < line x1 = "22" x2 = "2" y2 = "18" y1 = "18" ></ line > < line x1 = "6" x2 = "6" y2 = "22" y1 = "2" ></ line > < line x2 = "18" y1 = "2" x1 = "18" y2 = "22" ></ line > < / > } } pub const LUCIDE_FRAME : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor")] } ;