use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "10" x1 = "12" x2 = "12" y1 = "20" ></ line > < line y1 = "20" x2 = "18" y2 = "4" x1 = "18" ></ line > < line y2 = "16" x1 = "6" x2 = "6" y1 = "20" ></ line > < / > } } pub const LUCIDE_CHART_NO_AXES_COLUMN_INCREASING : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("height" , "24")] } ;