use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "14" x1 = "21" y2 = "4" y1 = "4" ></ line > < line x1 = "10" y2 = "4" y1 = "4" x2 = "3" ></ line > < line x1 = "21" y2 = "12" y1 = "12" x2 = "12" ></ line > < line x1 = "8" x2 = "3" y2 = "12" y1 = "12" ></ line > < line y2 = "20" x1 = "21" x2 = "16" y1 = "20" ></ line > < line x1 = "12" y1 = "20" y2 = "20" x2 = "3" ></ line > < line x2 = "14" y1 = "2" x1 = "14" y2 = "6" ></ line > < line y1 = "10" x2 = "8" y2 = "14" x1 = "8" ></ line > < line y1 = "18" x1 = "16" x2 = "16" y2 = "22" ></ line > < / > } } pub const LUCIDE_SLIDERS_HORIZONTAL : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;