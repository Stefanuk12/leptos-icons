use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "10" x1 = "12" x2 = "12" y1 = "20" ></ line > < line x2 = "18" y2 = "4" x1 = "18" y1 = "20" ></ line > < line x2 = "6" y1 = "20" y2 = "16" x1 = "6" ></ line > < / > } } pub const LUCIDE_CHART_NO_AXES_COLUMN_INCREASING : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("height" , "24")] } ;