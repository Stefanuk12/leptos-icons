use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "3" y1 = "6" x1 = "21" y2 = "6" ></ line > < line y1 = "12" y2 = "12" x1 = "17" x2 = "7" ></ line > < line y2 = "18" y1 = "18" x1 = "19" x2 = "5" ></ line > < / > } } pub const LUCIDE_ALIGN_CENTER : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;