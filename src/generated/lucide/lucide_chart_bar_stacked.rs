use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M11 13v4" ></ path > < path d = "M15 5v4" ></ path > < path d = "M3 3v16a2 2 0 0 0 2 2h16" ></ path > < rect rx = "1" width = "9" y = "13" x = "7" height = "4" ></ rect > < rect rx = "1" height = "4" x = "7" y = "5" width = "12" ></ rect > < / > } } pub const LUCIDE_CHART_BAR_STACKED : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linejoin" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke" , "currentColor")] } ;