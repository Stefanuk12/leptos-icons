use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "12" x2 = "12" y1 = "20" y2 = "10" ></ line > < line y1 = "20" y2 = "4" x1 = "18" x2 = "18" ></ line > < line x1 = "6" x2 = "6" y1 = "20" y2 = "16" ></ line > < / > } } pub const LUCIDE_CHART_NO_AXES_COLUMN_INCREASING : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24")] } ;