use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "4" x1 = "4" y1 = "21" y2 = "14" ></ line > < line x2 = "4" x1 = "4" y2 = "3" y1 = "10" ></ line > < line x1 = "12" x2 = "12" y2 = "12" y1 = "21" ></ line > < line x1 = "12" x2 = "12" y1 = "8" y2 = "3" ></ line > < line y1 = "21" y2 = "16" x1 = "20" x2 = "20" ></ line > < line x1 = "20" x2 = "20" y1 = "12" y2 = "3" ></ line > < line y1 = "14" x1 = "2" x2 = "6" y2 = "14" ></ line > < line y1 = "8" y2 = "8" x2 = "14" x1 = "10" ></ line > < line x2 = "22" y1 = "16" y2 = "16" x1 = "18" ></ line > < / > } } pub const LUCIDE_SLIDERS_VERTICAL : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("width" , "24")] } ;