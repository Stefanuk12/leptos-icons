use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "4" y1 = "4" x1 = "19" x2 = "10" ></ line > < line y2 = "20" y1 = "20" x2 = "5" x1 = "14" ></ line > < line y2 = "20" x1 = "15" x2 = "9" y1 = "4" ></ line > < / > } } pub const LUCIDE_ITALIC : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("width" , "24") , ("stroke" , "currentColor") , ("height" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round")] } ;