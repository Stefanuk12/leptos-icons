use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M13 17V9" ></ path > < path d = "M18 17V5" ></ path > < path d = "M3 3v16a2 2 0 0 0 2 2h16" ></ path > < path d = "M8 17v-3" ></ path > < / > } } pub const LUCIDE_CHART_COLUMN_INCREASING : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("fill" , "none") , ("height" , "24")] } ;