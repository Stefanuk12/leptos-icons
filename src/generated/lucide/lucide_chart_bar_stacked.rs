use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M11 13v4" ></ path > < path d = "M15 5v4" ></ path > < path d = "M3 3v16a2 2 0 0 0 2 2h16" ></ path > < rect rx = "1" width = "9" height = "4" x = "7" y = "13" ></ rect > < rect width = "12" height = "4" rx = "1" x = "7" y = "5" ></ rect > < / > } } pub const LUCIDE_CHART_BAR_STACKED : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round")] } ;