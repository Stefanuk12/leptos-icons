use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "20" y2 = "10" x1 = "18" x2 = "18" ></ line > < line y1 = "20" x1 = "12" x2 = "12" y2 = "4" ></ line > < line x1 = "6" y2 = "14" x2 = "6" y1 = "20" ></ line > < / > } } pub const LUCIDE_CHART_NO_AXES_COLUMN : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-linecap" , "round")] } ;