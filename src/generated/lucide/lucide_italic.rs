use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "19" y1 = "4" x2 = "10" y2 = "4" ></ line > < line x1 = "14" y2 = "20" y1 = "20" x2 = "5" ></ line > < line x1 = "15" x2 = "9" y1 = "4" y2 = "20" ></ line > < / > } } pub const LUCIDE_ITALIC : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("width" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;