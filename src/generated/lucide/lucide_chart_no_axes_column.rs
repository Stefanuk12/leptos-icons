use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "10" x1 = "18" y1 = "20" x2 = "18" ></ line > < line x2 = "12" y2 = "4" y1 = "20" x1 = "12" ></ line > < line x1 = "6" y2 = "14" y1 = "20" x2 = "6" ></ line > < / > } } pub const LUCIDE_CHART_NO_AXES_COLUMN : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke" , "currentColor")] } ;