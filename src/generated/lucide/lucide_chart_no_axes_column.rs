use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "20" x1 = "18" x2 = "18" y2 = "10" ></ line > < line y2 = "4" x2 = "12" y1 = "20" x1 = "12" ></ line > < line x1 = "6" x2 = "6" y1 = "20" y2 = "14" ></ line > < / > } } pub const LUCIDE_CHART_NO_AXES_COLUMN : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;