use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "4" y2 = "4" x2 = "14" x1 = "21" ></ line > < line y1 = "4" x2 = "3" y2 = "4" x1 = "10" ></ line > < line x2 = "12" y1 = "12" y2 = "12" x1 = "21" ></ line > < line x2 = "3" y1 = "12" x1 = "8" y2 = "12" ></ line > < line x2 = "16" y1 = "20" y2 = "20" x1 = "21" ></ line > < line x1 = "12" x2 = "3" y1 = "20" y2 = "20" ></ line > < line y1 = "2" x1 = "14" y2 = "6" x2 = "14" ></ line > < line y1 = "10" x1 = "8" x2 = "8" y2 = "14" ></ line > < line y2 = "22" x2 = "16" x1 = "16" y1 = "18" ></ line > < / > } } pub const LUCIDE_SLIDERS_HORIZONTAL : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("fill" , "none")] } ;