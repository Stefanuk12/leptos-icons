use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 3v18h18" ></ path > < rect rx = "1" x = "7" width = "12" y = "5" height = "4" ></ rect > < rect height = "4" y = "13" rx = "1" x = "7" width = "7" ></ rect > < / > } } pub const LUCIDE_BAR_CHART_HORIZONTAL_BIG : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24")] } ;