use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "6" x2 = "3" y1 = "6" x1 = "21" ></ line > < line x2 = "9" y2 = "12" y1 = "12" x1 = "21" ></ line > < line y2 = "18" x2 = "7" y1 = "18" x1 = "21" ></ line > < / > } } pub const LUCIDE_ALIGN_RIGHT : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;