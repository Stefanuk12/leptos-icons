use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 3v18h18" ></ path > < rect y = "5" width = "12" height = "4" rx = "1" x = "7" ></ rect > < rect rx = "1" height = "4" width = "7" x = "7" y = "13" ></ rect > < / > } } pub const LUCIDE_BAR_CHART_HORIZONTAL_BIG : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-width" , "2")] } ;