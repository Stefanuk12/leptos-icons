use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 20V10" ></ path > < path d = "M18 20v-4" ></ path > < path d = "M6 20V4" ></ path > < / > } } pub const LUCIDE_CHART_NO_AXES_COLUMN_DECREASING : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("width" , "24")] } ;