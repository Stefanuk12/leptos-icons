use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M11 13v4" ></ path > < path d = "M15 5v4" ></ path > < path d = "M3 3v16a2 2 0 0 0 2 2h16" ></ path > < rect height = "4" y = "13" x = "7" width = "9" rx = "1" ></ rect > < rect x = "7" y = "5" rx = "1" width = "12" height = "4" ></ rect > < / > } } pub const LUCIDE_CHART_BAR_STACKED : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;