use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 3v18h18" ></ path > < rect rx = "1" x = "7" width = "4" height = "7" y = "10" ></ rect > < rect height = "12" width = "4" y = "5" rx = "1" x = "15" ></ rect > < / > } } pub const LUCIDE_BAR_CHART_BIG : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;