use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "4" x2 = "4" y1 = "21" y2 = "14" ></ line > < line y1 = "10" x1 = "4" y2 = "3" x2 = "4" ></ line > < line x2 = "12" y2 = "12" y1 = "21" x1 = "12" ></ line > < line x1 = "12" y1 = "8" x2 = "12" y2 = "3" ></ line > < line x1 = "20" x2 = "20" y2 = "16" y1 = "21" ></ line > < line y2 = "3" y1 = "12" x2 = "20" x1 = "20" ></ line > < line x2 = "6" y2 = "14" y1 = "14" x1 = "2" ></ line > < line y2 = "8" y1 = "8" x1 = "10" x2 = "14" ></ line > < line x1 = "18" y2 = "16" y1 = "16" x2 = "22" ></ line > < / > } } pub const LUCIDE_SLIDERS_VERTICAL : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;