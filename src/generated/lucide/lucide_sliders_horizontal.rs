use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "21" x2 = "14" y1 = "4" y2 = "4" ></ line > < line x1 = "10" x2 = "3" y1 = "4" y2 = "4" ></ line > < line x1 = "21" x2 = "12" y2 = "12" y1 = "12" ></ line > < line x2 = "3" y1 = "12" y2 = "12" x1 = "8" ></ line > < line y2 = "20" y1 = "20" x1 = "21" x2 = "16" ></ line > < line x1 = "12" y1 = "20" y2 = "20" x2 = "3" ></ line > < line x2 = "14" y2 = "6" y1 = "2" x1 = "14" ></ line > < line x1 = "8" y1 = "10" x2 = "8" y2 = "14" ></ line > < line x2 = "16" y1 = "18" y2 = "22" x1 = "16" ></ line > < / > } } pub const LUCIDE_SLIDERS_HORIZONTAL : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;