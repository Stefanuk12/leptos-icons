use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 3v18h18" ></ path > < rect x = "7" rx = "1" width = "4" y = "10" height = "7" ></ rect > < rect y = "5" x = "15" width = "4" rx = "1" height = "12" ></ rect > < / > } } pub const LUCIDE_BAR_CHART_BIG : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("height" , "24") , ("width" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round")] } ;