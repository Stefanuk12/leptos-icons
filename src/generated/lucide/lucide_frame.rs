use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "22" y2 = "6" x2 = "2" y1 = "6" ></ line > < line x1 = "22" y1 = "18" x2 = "2" y2 = "18" ></ line > < line y2 = "22" x1 = "6" y1 = "2" x2 = "6" ></ line > < line y1 = "2" y2 = "22" x1 = "18" x2 = "18" ></ line > < / > } } pub const LUCIDE_FRAME : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;