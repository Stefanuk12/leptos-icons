use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "10" x1 = "19" y2 = "4" y1 = "4" ></ line > < line y2 = "20" y1 = "20" x1 = "14" x2 = "5" ></ line > < line y1 = "4" x2 = "9" x1 = "15" y2 = "20" ></ line > < / > } } pub const LUCIDE_ITALIC : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;