use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "21" y1 = "4" x2 = "14" y2 = "4" ></ line > < line x2 = "3" y2 = "4" y1 = "4" x1 = "10" ></ line > < line y1 = "12" x2 = "12" x1 = "21" y2 = "12" ></ line > < line y1 = "12" y2 = "12" x1 = "8" x2 = "3" ></ line > < line x1 = "21" y2 = "20" x2 = "16" y1 = "20" ></ line > < line x1 = "12" x2 = "3" y1 = "20" y2 = "20" ></ line > < line y2 = "6" x2 = "14" x1 = "14" y1 = "2" ></ line > < line y1 = "10" x1 = "8" y2 = "14" x2 = "8" ></ line > < line x1 = "16" y1 = "18" x2 = "16" y2 = "22" ></ line > < / > } } pub const LUCIDE_SLIDERS_HORIZONTAL : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;