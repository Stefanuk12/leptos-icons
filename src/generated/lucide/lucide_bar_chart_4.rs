use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 3v18h18" ></ path > < path d = "M13 17V9" ></ path > < path d = "M18 17V5" ></ path > < path d = "M8 17v-3" ></ path > < / > } } pub const LUCIDE_BAR_CHART_4 : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("height" , "24")] } ;