use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "3" y1 = "6" x1 = "21" y2 = "6" ></ line > < line y2 = "12" x2 = "9" y1 = "12" x1 = "21" ></ line > < line x2 = "7" x1 = "21" y1 = "18" y2 = "18" ></ line > < / > } } pub const LUCIDE_ALIGN_RIGHT : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;