use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 5v4" ></ path > < rect height = "6" y = "9" rx = "1" width = "4" x = "7" ></ rect > < path d = "M9 15v2" ></ path > < path d = "M17 3v2" ></ path > < rect x = "15" height = "8" y = "5" width = "4" rx = "1" ></ rect > < path d = "M17 13v3" ></ path > < path d = "M3 3v16a2 2 0 0 0 2 2h16" ></ path > < / > } } pub const LUCIDE_CHART_CANDLESTICK : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("height" , "24")] } ;