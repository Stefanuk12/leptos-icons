use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "12" y1 = "20" y2 = "10" x2 = "12" ></ line > < line x2 = "18" y2 = "4" x1 = "18" y1 = "20" ></ line > < line x1 = "6" y1 = "20" x2 = "6" y2 = "16" ></ line > < / > } } pub const LUCIDE_BAR_CHART : Path = Path { path : icon_path , props : & [("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;