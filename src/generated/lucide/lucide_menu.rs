use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "4" x2 = "20" y2 = "12" y1 = "12" ></ line > < line y2 = "6" x2 = "20" x1 = "4" y1 = "6" ></ line > < line y2 = "18" y1 = "18" x1 = "4" x2 = "20" ></ line > < / > } } pub const LUCIDE_MENU : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;