use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "19" y1 = "4" x2 = "10" y2 = "4" ></ line > < line x2 = "5" x1 = "14" y2 = "20" y1 = "20" ></ line > < line y2 = "20" x1 = "15" y1 = "4" x2 = "9" ></ line > < / > } } pub const LUCIDE_ITALIC : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;