use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "20" x1 = "12" y2 = "10" x2 = "12" ></ line > < line x1 = "18" y2 = "4" x2 = "18" y1 = "20" ></ line > < line x2 = "6" y1 = "20" y2 = "16" x1 = "6" ></ line > < / > } } pub const LUCIDE_BAR_CHART : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;