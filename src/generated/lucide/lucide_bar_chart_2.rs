use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "18" x1 = "18" y2 = "10" y1 = "20" ></ line > < line x2 = "12" x1 = "12" y2 = "4" y1 = "20" ></ line > < line x2 = "6" y1 = "20" x1 = "6" y2 = "14" ></ line > < / > } } pub const LUCIDE_BAR_CHART_2 : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;