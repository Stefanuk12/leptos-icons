use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "6" y2 = "6" x2 = "21" x1 = "3" ></ line > < line y2 = "12" x2 = "21" y1 = "12" x1 = "3" ></ line > < line x1 = "3" y1 = "18" y2 = "18" x2 = "21" ></ line > < / > } } pub const LUCIDE_ALIGN_JUSTIFY : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;