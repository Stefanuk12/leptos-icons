use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "12" y2 = "10" y1 = "20" x2 = "12" ></ line > < line x1 = "18" y2 = "4" x2 = "18" y1 = "20" ></ line > < line y2 = "16" x2 = "6" y1 = "20" x1 = "6" ></ line > < / > } } pub const LUCIDE_BAR_CHART : Path = Path { path : icon_path , props : & [("fill" , "none") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("height" , "24")] } ;