use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "12" y1 = "20" y2 = "10" x1 = "12" ></ line > < line y1 = "20" y2 = "4" x2 = "18" x1 = "18" ></ line > < line y1 = "20" x1 = "6" x2 = "6" y2 = "16" ></ line > < / > } } pub const LUCIDE_CHART_NO_AXES_COLUMN_INCREASING : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;