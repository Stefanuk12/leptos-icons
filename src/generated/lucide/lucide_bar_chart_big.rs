use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 3v18h18" ></ path > < rect height = "7" x = "7" y = "10" width = "4" rx = "1" ></ rect > < rect x = "15" rx = "1" width = "4" y = "5" height = "12" ></ rect > < / > } } pub const LUCIDE_BAR_CHART_BIG : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;