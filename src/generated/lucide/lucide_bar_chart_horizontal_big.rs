use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 3v18h18" ></ path > < rect y = "5" height = "4" rx = "1" width = "12" x = "7" ></ rect > < rect height = "4" y = "13" rx = "1" width = "7" x = "7" ></ rect > < / > } } pub const LUCIDE_BAR_CHART_HORIZONTAL_BIG : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("width" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round")] } ;