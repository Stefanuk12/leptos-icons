use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "6" y2 = "6" x1 = "21" x2 = "3" ></ line > < line y2 = "12" x2 = "7" y1 = "12" x1 = "17" ></ line > < line x1 = "19" x2 = "5" y2 = "18" y1 = "18" ></ line > < / > } } pub const LUCIDE_ALIGN_CENTER : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;