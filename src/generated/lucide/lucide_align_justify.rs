use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "6" x1 = "3" x2 = "21" y2 = "6" ></ line > < line y1 = "12" x1 = "3" y2 = "12" x2 = "21" ></ line > < line y2 = "18" x2 = "21" y1 = "18" x1 = "3" ></ line > < / > } } pub const LUCIDE_ALIGN_JUSTIFY : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor")] } ;