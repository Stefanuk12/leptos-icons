use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "12" x2 = "20" x1 = "4" y2 = "12" ></ line > < line x2 = "20" x1 = "4" y2 = "6" y1 = "6" ></ line > < line x1 = "4" y1 = "18" x2 = "20" y2 = "18" ></ line > < / > } } pub const LUCIDE_MENU : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("width" , "24")] } ;