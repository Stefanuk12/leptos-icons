use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "12" x2 = "12" y1 = "20" y2 = "10" ></ line > < line x2 = "18" x1 = "18" y1 = "20" y2 = "4" ></ line > < line y2 = "16" x1 = "6" x2 = "6" y1 = "20" ></ line > < / > } } pub const LUCIDE_CHART_NO_AXES_COLUMN_INCREASING : Path = Path { path : icon_path , props : & [("fill" , "none") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2")] } ;