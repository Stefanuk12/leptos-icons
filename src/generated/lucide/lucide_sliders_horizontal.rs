use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "4" x2 = "14" y2 = "4" x1 = "21" ></ line > < line y2 = "4" x2 = "3" x1 = "10" y1 = "4" ></ line > < line x2 = "12" y2 = "12" y1 = "12" x1 = "21" ></ line > < line y1 = "12" x2 = "3" y2 = "12" x1 = "8" ></ line > < line x1 = "21" x2 = "16" y1 = "20" y2 = "20" ></ line > < line x1 = "12" y1 = "20" x2 = "3" y2 = "20" ></ line > < line x2 = "14" y1 = "2" y2 = "6" x1 = "14" ></ line > < line y2 = "14" x2 = "8" x1 = "8" y1 = "10" ></ line > < line x2 = "16" y1 = "18" x1 = "16" y2 = "22" ></ line > < / > } } pub const LUCIDE_SLIDERS_HORIZONTAL : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none") , ("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;