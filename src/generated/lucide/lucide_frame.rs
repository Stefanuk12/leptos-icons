use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "2" y1 = "6" y2 = "6" x1 = "22" ></ line > < line y2 = "18" y1 = "18" x1 = "22" x2 = "2" ></ line > < line y2 = "22" x1 = "6" x2 = "6" y1 = "2" ></ line > < line y2 = "22" x2 = "18" x1 = "18" y1 = "2" ></ line > < / > } } pub const LUCIDE_FRAME : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("height" , "24")] } ;