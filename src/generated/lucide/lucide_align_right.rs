use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "3" x1 = "21" y1 = "6" y2 = "6" ></ line > < line y1 = "12" y2 = "12" x1 = "21" x2 = "9" ></ line > < line x2 = "7" x1 = "21" y2 = "18" y1 = "18" ></ line > < / > } } pub const LUCIDE_ALIGN_RIGHT : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-linecap" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2")] } ;