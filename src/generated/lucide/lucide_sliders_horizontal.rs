use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "21" y2 = "4" y1 = "4" x2 = "14" ></ line > < line y1 = "4" y2 = "4" x2 = "3" x1 = "10" ></ line > < line y1 = "12" x1 = "21" x2 = "12" y2 = "12" ></ line > < line y2 = "12" y1 = "12" x1 = "8" x2 = "3" ></ line > < line y2 = "20" x1 = "21" x2 = "16" y1 = "20" ></ line > < line y2 = "20" y1 = "20" x1 = "12" x2 = "3" ></ line > < line x1 = "14" y1 = "2" y2 = "6" x2 = "14" ></ line > < line y2 = "14" x1 = "8" x2 = "8" y1 = "10" ></ line > < line y2 = "22" x2 = "16" y1 = "18" x1 = "16" ></ line > < / > } } pub const LUCIDE_SLIDERS_HORIZONTAL : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("fill" , "none")] } ;