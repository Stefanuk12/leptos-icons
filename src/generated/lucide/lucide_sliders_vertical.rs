use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "14" x1 = "4" y1 = "21" x2 = "4" ></ line > < line x2 = "4" x1 = "4" y2 = "3" y1 = "10" ></ line > < line x1 = "12" y2 = "12" y1 = "21" x2 = "12" ></ line > < line x1 = "12" y2 = "3" x2 = "12" y1 = "8" ></ line > < line y1 = "21" x2 = "20" x1 = "20" y2 = "16" ></ line > < line y1 = "12" y2 = "3" x2 = "20" x1 = "20" ></ line > < line x1 = "2" y2 = "14" x2 = "6" y1 = "14" ></ line > < line x2 = "14" x1 = "10" y2 = "8" y1 = "8" ></ line > < line y2 = "16" x2 = "22" x1 = "18" y1 = "16" ></ line > < / > } } pub const LUCIDE_SLIDERS_VERTICAL : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("width" , "24")] } ;