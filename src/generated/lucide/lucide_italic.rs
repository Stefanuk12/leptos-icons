use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "19" y1 = "4" x2 = "10" y2 = "4" ></ line > < line y2 = "20" y1 = "20" x2 = "5" x1 = "14" ></ line > < line x2 = "9" y2 = "20" y1 = "4" x1 = "15" ></ line > < / > } } pub const LUCIDE_ITALIC : Path = Path { path : icon_path , props : & [("fill" , "none") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;