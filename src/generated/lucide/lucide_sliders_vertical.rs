use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "14" y1 = "21" x2 = "4" x1 = "4" ></ line > < line x1 = "4" x2 = "4" y2 = "3" y1 = "10" ></ line > < line x2 = "12" y1 = "21" x1 = "12" y2 = "12" ></ line > < line x2 = "12" x1 = "12" y2 = "3" y1 = "8" ></ line > < line y2 = "16" y1 = "21" x2 = "20" x1 = "20" ></ line > < line y1 = "12" x1 = "20" y2 = "3" x2 = "20" ></ line > < line y1 = "14" x2 = "6" x1 = "2" y2 = "14" ></ line > < line y1 = "8" x2 = "14" y2 = "8" x1 = "10" ></ line > < line y1 = "16" x2 = "22" y2 = "16" x1 = "18" ></ line > < / > } } pub const LUCIDE_SLIDERS_VERTICAL : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;