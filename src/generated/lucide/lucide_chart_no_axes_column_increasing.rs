use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "12" y2 = "10" y1 = "20" x1 = "12" ></ line > < line y2 = "4" y1 = "20" x2 = "18" x1 = "18" ></ line > < line x1 = "6" y1 = "20" y2 = "16" x2 = "6" ></ line > < / > } } pub const LUCIDE_CHART_NO_AXES_COLUMN_INCREASING : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("width" , "24")] } ;