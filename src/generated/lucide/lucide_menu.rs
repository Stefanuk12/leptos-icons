use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "12" x2 = "20" y2 = "12" x1 = "4" ></ line > < line y1 = "6" y2 = "6" x2 = "20" x1 = "4" ></ line > < line x2 = "20" y1 = "18" y2 = "18" x1 = "4" ></ line > < / > } } pub const LUCIDE_MENU : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;