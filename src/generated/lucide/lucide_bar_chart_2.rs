use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "18" y2 = "10" x1 = "18" y1 = "20" ></ line > < line y2 = "4" y1 = "20" x1 = "12" x2 = "12" ></ line > < line y1 = "20" x2 = "6" x1 = "6" y2 = "14" ></ line > < / > } } pub const LUCIDE_BAR_CHART_2 : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-width" , "2")] } ;