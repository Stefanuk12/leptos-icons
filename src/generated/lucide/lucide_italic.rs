use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "19" y1 = "4" x2 = "10" y2 = "4" ></ line > < line y1 = "20" x2 = "5" x1 = "14" y2 = "20" ></ line > < line x2 = "9" y2 = "20" y1 = "4" x1 = "15" ></ line > < / > } } pub const LUCIDE_ITALIC : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;