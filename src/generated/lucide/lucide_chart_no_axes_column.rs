use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "18" y2 = "10" y1 = "20" x1 = "18" ></ line > < line x2 = "12" y2 = "4" y1 = "20" x1 = "12" ></ line > < line x1 = "6" x2 = "6" y2 = "14" y1 = "20" ></ line > < / > } } pub const LUCIDE_CHART_NO_AXES_COLUMN : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24")] } ;