use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "4" x2 = "10" x1 = "19" y1 = "4" ></ line > < line x2 = "5" y1 = "20" y2 = "20" x1 = "14" ></ line > < line x2 = "9" y2 = "20" x1 = "15" y1 = "4" ></ line > < / > } } pub const LUCIDE_ITALIC : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("width" , "24") , ("stroke-width" , "2")] } ;