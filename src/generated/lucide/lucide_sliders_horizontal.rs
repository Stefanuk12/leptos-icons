use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "21" y1 = "4" x2 = "14" y2 = "4" ></ line > < line x2 = "3" x1 = "10" y1 = "4" y2 = "4" ></ line > < line x1 = "21" y1 = "12" y2 = "12" x2 = "12" ></ line > < line y2 = "12" x1 = "8" x2 = "3" y1 = "12" ></ line > < line y2 = "20" x2 = "16" x1 = "21" y1 = "20" ></ line > < line y2 = "20" x2 = "3" y1 = "20" x1 = "12" ></ line > < line y1 = "2" y2 = "6" x1 = "14" x2 = "14" ></ line > < line x1 = "8" y1 = "10" y2 = "14" x2 = "8" ></ line > < line x1 = "16" y1 = "18" y2 = "22" x2 = "16" ></ line > < / > } } pub const LUCIDE_SLIDERS_HORIZONTAL : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none")] } ;