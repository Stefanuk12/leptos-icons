use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "4" y1 = "21" x2 = "4" y2 = "14" ></ line > < line x2 = "4" y1 = "10" y2 = "3" x1 = "4" ></ line > < line y1 = "21" y2 = "12" x2 = "12" x1 = "12" ></ line > < line x1 = "12" x2 = "12" y1 = "8" y2 = "3" ></ line > < line x2 = "20" y1 = "21" x1 = "20" y2 = "16" ></ line > < line y1 = "12" x2 = "20" x1 = "20" y2 = "3" ></ line > < line x1 = "2" x2 = "6" y1 = "14" y2 = "14" ></ line > < line y1 = "8" x1 = "10" y2 = "8" x2 = "14" ></ line > < line y2 = "16" x2 = "22" x1 = "18" y1 = "16" ></ line > < / > } } pub const LUCIDE_SLIDERS_VERTICAL : Path = Path { path : icon_path , props : & [("width" , "24") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor")] } ;