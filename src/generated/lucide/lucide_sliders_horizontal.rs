use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "4" x1 = "21" y1 = "4" x2 = "14" ></ line > < line x2 = "3" x1 = "10" y1 = "4" y2 = "4" ></ line > < line y2 = "12" y1 = "12" x1 = "21" x2 = "12" ></ line > < line x1 = "8" y2 = "12" x2 = "3" y1 = "12" ></ line > < line x1 = "21" y1 = "20" y2 = "20" x2 = "16" ></ line > < line x1 = "12" y1 = "20" y2 = "20" x2 = "3" ></ line > < line x1 = "14" x2 = "14" y1 = "2" y2 = "6" ></ line > < line x2 = "8" x1 = "8" y1 = "10" y2 = "14" ></ line > < line x1 = "16" x2 = "16" y1 = "18" y2 = "22" ></ line > < / > } } pub const LUCIDE_SLIDERS_HORIZONTAL : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round")] } ;