use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 3v16a2 2 0 0 0 2 2h16" ></ path > < rect rx = "1" x = "7" width = "9" y = "13" height = "4" ></ rect > < rect x = "7" y = "5" height = "4" rx = "1" width = "12" ></ rect > < / > } } pub const LUCIDE_CHART_BAR_BIG : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;