use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 3v16a2 2 0 0 0 2 2h16" ></ path > < rect x = "7" height = "4" width = "9" rx = "1" y = "13" ></ rect > < rect x = "7" height = "4" width = "12" y = "5" rx = "1" ></ rect > < / > } } pub const LUCIDE_CHART_BAR_BIG : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24")] } ;