use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "18" y1 = "20" x1 = "18" y2 = "10" ></ line > < line y2 = "4" x1 = "12" x2 = "12" y1 = "20" ></ line > < line x2 = "6" y2 = "14" x1 = "6" y1 = "20" ></ line > < / > } } pub const LUCIDE_BAR_CHART_2 : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round")] } ;