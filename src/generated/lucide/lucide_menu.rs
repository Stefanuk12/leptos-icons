use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "12" x1 = "4" x2 = "20" y2 = "12" ></ line > < line x2 = "20" y2 = "6" x1 = "4" y1 = "6" ></ line > < line x1 = "4" y2 = "18" y1 = "18" x2 = "20" ></ line > < / > } } pub const LUCIDE_MENU : Path = Path { path : icon_path , props : & [("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;