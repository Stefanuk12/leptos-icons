use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 3v16a2 2 0 0 0 2 2h16" ></ path > < rect rx = "1" y = "13" height = "4" x = "7" width = "9" ></ rect > < rect x = "7" width = "12" height = "4" y = "5" rx = "1" ></ rect > < / > } } pub const LUCIDE_CHART_BAR_BIG : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("width" , "24")] } ;