use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "4" y2 = "14" x1 = "4" y1 = "21" ></ line > < line x2 = "4" y2 = "3" x1 = "4" y1 = "10" ></ line > < line x2 = "12" x1 = "12" y2 = "12" y1 = "21" ></ line > < line x2 = "12" y2 = "3" x1 = "12" y1 = "8" ></ line > < line x2 = "20" y1 = "21" x1 = "20" y2 = "16" ></ line > < line y2 = "3" y1 = "12" x1 = "20" x2 = "20" ></ line > < line x2 = "6" x1 = "2" y1 = "14" y2 = "14" ></ line > < line x1 = "10" x2 = "14" y1 = "8" y2 = "8" ></ line > < line x1 = "18" y1 = "16" y2 = "16" x2 = "22" ></ line > < / > } } pub const LUCIDE_SLIDERS_VERTICAL : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("width" , "24") , ("height" , "24")] } ;