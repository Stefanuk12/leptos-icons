use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "4" y1 = "21" x2 = "4" y2 = "14" ></ line > < line x2 = "4" y1 = "10" y2 = "3" x1 = "4" ></ line > < line x2 = "12" x1 = "12" y1 = "21" y2 = "12" ></ line > < line x1 = "12" y1 = "8" x2 = "12" y2 = "3" ></ line > < line y1 = "21" x1 = "20" x2 = "20" y2 = "16" ></ line > < line x1 = "20" x2 = "20" y2 = "3" y1 = "12" ></ line > < line y1 = "14" x1 = "2" y2 = "14" x2 = "6" ></ line > < line x2 = "14" y1 = "8" y2 = "8" x1 = "10" ></ line > < line y1 = "16" x1 = "18" x2 = "22" y2 = "16" ></ line > < / > } } pub const LUCIDE_SLIDERS_VERTICAL : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("height" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round")] } ;