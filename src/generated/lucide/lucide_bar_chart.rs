use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "12" y1 = "20" y2 = "10" x1 = "12" ></ line > < line y1 = "20" x2 = "18" x1 = "18" y2 = "4" ></ line > < line x2 = "6" x1 = "6" y1 = "20" y2 = "16" ></ line > < / > } } pub const LUCIDE_BAR_CHART : Path = Path { path : icon_path , props : & [("fill" , "none") , ("width" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke-width" , "2")] } ;