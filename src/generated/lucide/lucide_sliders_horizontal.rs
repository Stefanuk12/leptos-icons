use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "21" x2 = "14" y1 = "4" y2 = "4" ></ line > < line y1 = "4" y2 = "4" x1 = "10" x2 = "3" ></ line > < line y2 = "12" x1 = "21" x2 = "12" y1 = "12" ></ line > < line y2 = "12" y1 = "12" x1 = "8" x2 = "3" ></ line > < line x2 = "16" y2 = "20" y1 = "20" x1 = "21" ></ line > < line x2 = "3" y1 = "20" x1 = "12" y2 = "20" ></ line > < line y2 = "6" x2 = "14" y1 = "2" x1 = "14" ></ line > < line x2 = "8" y1 = "10" y2 = "14" x1 = "8" ></ line > < line y2 = "22" y1 = "18" x1 = "16" x2 = "16" ></ line > < / > } } pub const LUCIDE_SLIDERS_HORIZONTAL : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;