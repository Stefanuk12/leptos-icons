use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "12" x2 = "12" y2 = "10" y1 = "20" ></ line > < line y1 = "20" x2 = "18" x1 = "18" y2 = "4" ></ line > < line x1 = "6" y1 = "20" x2 = "6" y2 = "16" ></ line > < / > } } pub const LUCIDE_BAR_CHART : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("width" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;