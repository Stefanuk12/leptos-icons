use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "4" y1 = "4" x1 = "21" x2 = "14" ></ line > < line x1 = "10" x2 = "3" y2 = "4" y1 = "4" ></ line > < line x2 = "12" x1 = "21" y2 = "12" y1 = "12" ></ line > < line x2 = "3" y1 = "12" x1 = "8" y2 = "12" ></ line > < line y2 = "20" y1 = "20" x1 = "21" x2 = "16" ></ line > < line x1 = "12" x2 = "3" y1 = "20" y2 = "20" ></ line > < line y2 = "6" x1 = "14" y1 = "2" x2 = "14" ></ line > < line y1 = "10" x2 = "8" x1 = "8" y2 = "14" ></ line > < line y1 = "18" x2 = "16" x1 = "16" y2 = "22" ></ line > < / > } } pub const LUCIDE_SLIDERS_HORIZONTAL : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("width" , "24")] } ;