use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M11 13v4" ></ path > < path d = "M15 5v4" ></ path > < path d = "M3 3v16a2 2 0 0 0 2 2h16" ></ path > < rect width = "9" x = "7" y = "13" height = "4" rx = "1" ></ rect > < rect rx = "1" width = "12" x = "7" y = "5" height = "4" ></ rect > < / > } } pub const LUCIDE_CHART_BAR_STACKED : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("height" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;