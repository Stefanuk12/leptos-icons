use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "21" y2 = "4" x2 = "14" y1 = "4" ></ line > < line y1 = "4" x2 = "3" x1 = "10" y2 = "4" ></ line > < line y2 = "12" x2 = "12" x1 = "21" y1 = "12" ></ line > < line x1 = "8" x2 = "3" y2 = "12" y1 = "12" ></ line > < line y1 = "20" x2 = "16" y2 = "20" x1 = "21" ></ line > < line x2 = "3" x1 = "12" y1 = "20" y2 = "20" ></ line > < line y2 = "6" y1 = "2" x1 = "14" x2 = "14" ></ line > < line y1 = "10" x1 = "8" x2 = "8" y2 = "14" ></ line > < line y2 = "22" x2 = "16" x1 = "16" y1 = "18" ></ line > < / > } } pub const LUCIDE_SLIDERS_HORIZONTAL : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-width" , "2")] } ;