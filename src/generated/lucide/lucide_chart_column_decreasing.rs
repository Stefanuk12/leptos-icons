use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M13 17V9" ></ path > < path d = "M18 17v-3" ></ path > < path d = "M3 3v16a2 2 0 0 0 2 2h16" ></ path > < path d = "M8 17V5" ></ path > < / > } } pub const LUCIDE_CHART_COLUMN_DECREASING : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round")] } ;