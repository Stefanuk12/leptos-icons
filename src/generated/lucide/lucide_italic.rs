use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "4" y2 = "4" x1 = "19" x2 = "10" ></ line > < line y1 = "20" x1 = "14" x2 = "5" y2 = "20" ></ line > < line y2 = "20" y1 = "4" x2 = "9" x1 = "15" ></ line > < / > } } pub const LUCIDE_ITALIC : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none")] } ;