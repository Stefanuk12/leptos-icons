use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "6" y1 = "6" x1 = "22" x2 = "2" ></ line > < line y2 = "18" y1 = "18" x1 = "22" x2 = "2" ></ line > < line x1 = "6" y2 = "22" y1 = "2" x2 = "6" ></ line > < line y1 = "2" x1 = "18" x2 = "18" y2 = "22" ></ line > < / > } } pub const LUCIDE_FRAME : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;