use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M13 17V9" ></ path > < path d = "M18 17v-3" ></ path > < path d = "M3 3v16a2 2 0 0 0 2 2h16" ></ path > < path d = "M8 17V5" ></ path > < / > } } pub const LUCIDE_CHART_COLUMN_DECREASING : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24")] } ;