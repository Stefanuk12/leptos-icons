use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "10" x1 = "18" y1 = "20" x2 = "18" ></ line > < line x1 = "12" x2 = "12" y2 = "4" y1 = "20" ></ line > < line y1 = "20" x2 = "6" y2 = "14" x1 = "6" ></ line > < / > } } pub const LUCIDE_BAR_CHART_2 : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-width" , "2")] } ;