use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "21" y2 = "14" x1 = "4" x2 = "4" ></ line > < line x2 = "4" y2 = "3" x1 = "4" y1 = "10" ></ line > < line x1 = "12" x2 = "12" y2 = "12" y1 = "21" ></ line > < line y2 = "3" x1 = "12" y1 = "8" x2 = "12" ></ line > < line y1 = "21" x1 = "20" y2 = "16" x2 = "20" ></ line > < line y2 = "3" x2 = "20" y1 = "12" x1 = "20" ></ line > < line y1 = "14" y2 = "14" x1 = "2" x2 = "6" ></ line > < line x2 = "14" y2 = "8" y1 = "8" x1 = "10" ></ line > < line y2 = "16" x1 = "18" x2 = "22" y1 = "16" ></ line > < / > } } pub const LUCIDE_SLIDERS_VERTICAL : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("height" , "24")] } ;