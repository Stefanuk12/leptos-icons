use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "6" x1 = "22" x2 = "2" y2 = "6" ></ line > < line y1 = "18" x1 = "22" y2 = "18" x2 = "2" ></ line > < line x1 = "6" x2 = "6" y1 = "2" y2 = "22" ></ line > < line y1 = "2" x1 = "18" x2 = "18" y2 = "22" ></ line > < / > } } pub const LUCIDE_FRAME : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24")] } ;