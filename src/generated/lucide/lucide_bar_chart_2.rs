use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "20" x1 = "18" x2 = "18" y2 = "10" ></ line > < line x1 = "12" y1 = "20" y2 = "4" x2 = "12" ></ line > < line y2 = "14" y1 = "20" x1 = "6" x2 = "6" ></ line > < / > } } pub const LUCIDE_BAR_CHART_2 : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke" , "currentColor")] } ;